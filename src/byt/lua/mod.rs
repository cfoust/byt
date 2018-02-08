// EXTERNS

// LIBRARY INCLUDES
use hlua;
use hlua::{
    Lua,
    LuaError
};

// SUBMODULES
mod editor;

// LOCAL INCLUDES
use byt::editor::Editor;

#[derive(Debug)]
/// This is a hack. There's a good reason for it though. The goal with executing Lua code is that
/// we don't want Lua to allocate new objects and keep them as userdata. Rather, we'd like to be
/// able to shove in whatever pointers we want and have the Lua execution context use those
/// instead. This is converted back into a real mutable pointer to some kind of data -- usually a
/// struct internal to byt like Editor or FileView -- and then operated upon. hlua, the crate we
/// use for Lua, doesn't allow for u64, so we have to break the pointer into its two halves
/// (assuming we're on a 64-bit platform, but frankly I don't have any interest in supporting
/// 32-bit).
///
/// Everything that operates on these is incredibly unsafe. I'm aware that this is unidiomatic and
/// potentially dangerous, but it was the means to an end. In the future I can come in and fix it
/// to be a bit less frightening, but right now it works and it's not _that_ insecure.
struct HackyPtr {
    top_bits    : u32,
    bottom_bits : u32,
}

impl HackyPtr {
    /// Make a hacky pointer for an arbitrary type.
    fn new<T>(target : &T) -> HackyPtr {
        let address     = target as *const T as usize;
        let top_bits    = (address >> 32) as u32;
        // Cut out the top bits by truncating them
        let bottom_bits = address as u32 as usize as u32;

        HackyPtr { top_bits, bottom_bits }
    }

    /// Convert the HackyPtr to a mutable raw pointer.
    unsafe fn demarshall<T>(&self) -> *mut T {
        (((self.top_bits as u64) << 32) | (self.bottom_bits as u64))as *mut T
    }
}

// These allow hlua to push and pop the pointer type.
implement_lua_push!(HackyPtr, |mut metatable| {});
implement_lua_read!(HackyPtr);

/// Shove a pointer to an arbitrary type into some global name in the
/// Lua context.
fn marshall<T>(lua : &mut Lua, name : &str, target : &T) {
    lua.set(name, HackyPtr::new(target));
}

/// Initialize a Lua VM with everything necessary to interoperate with byt.
pub fn init_lua(mut lua : &mut Lua) -> Result<(), LuaError> {
    lua.openlibs();

    editor::init_lua_editor(lua)?;

    // ENTER marshall(&mut lua, "__editor", editor);

    //lua.execute::<()>(r#"
    //Editor:open('testfiles/no_line_ending.txt')
    //"#)
        //.unwrap();

    Ok(())
}
