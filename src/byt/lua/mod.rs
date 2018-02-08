// EXTERNS

// LIBRARY INCLUDES
use hlua;
use hlua::{Lua, LuaTable};

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::Editor;

#[derive(Debug)]
struct HackyPtr {
    top_bits    : u32,
    bottom_bits : u32,
}

impl HackyPtr {
    fn new<T>(target : &T) -> HackyPtr {
        let address     = target as *const T as usize;
        let top_bits    = (address >> 32) as u32;
        // Cut out the top bits by truncating them
        let bottom_bits = address as u32 as usize as u32;

        HackyPtr { top_bits, bottom_bits }
    }

    unsafe fn demarshall<T>(&self) -> *mut T {
        (((self.top_bits as u64) << 32) | (self.bottom_bits as u64))as *mut T
    }
}

implement_lua_push!(HackyPtr, |mut metatable| {
    let mut index = metatable.empty_array("__index");

    index.set("top", hlua::function1(|ptr: &HackyPtr| {
        ptr.top_bits
    }));

    index.set("bottom", hlua::function1(|ptr: &HackyPtr| {
        ptr.bottom_bits
    }));
});
implement_lua_read!(HackyPtr);

fn marshall<T>(lua : &mut Lua, name : &str, target : &T) {
    lua.set(name, HackyPtr::new(target));
}

pub fn init_lua(editor : &Editor) {
    let mut lua = Lua::new();
    lua.openlibs();

    marshall(&mut lua, "__editor", editor);

    lua.set("__editor_open", hlua::function2(|ptr : &HackyPtr, path : hlua::AnyLuaValue| {
        unsafe {
            let mut editor = &mut *ptr.demarshall::<Editor>();
            if let hlua::AnyLuaValue::LuaString(path_str) = path {
                editor.open(path_str.as_str());
            }
        }
    }));

    assert_eq!(editor.files.len(), 0);

    lua.execute::<()>(r#"
    Editor = {}
    function Editor:open(path)
        __editor_open(__editor, path)
    end
    "#)
        .unwrap();

    lua.execute::<()>(r#"
    Editor:open('testfiles/no_line_ending.txt')
    "#)
        .unwrap();

    assert_eq!(editor.files.len(), 1);

    ::std::process::exit(0);
}
