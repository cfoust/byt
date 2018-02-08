// EXTERNS

// LIBRARY INCLUDES
use hlua;
use hlua::{
    Lua,
    LuaError
};

// SUBMODULES

// LOCAL INCLUDES
use super::HackyPtr;
use byt::editor::Editor;

pub fn init_lua_editor(mut lua : &mut Lua) -> Result<(), LuaError> {
    lua.set("__editor_open", hlua::function2(|ptr : &HackyPtr, path : hlua::AnyLuaValue| {
        unsafe {
            let mut editor = &mut *ptr.demarshall::<Editor>();

            if let hlua::AnyLuaValue::LuaString(path_str) = path {
                editor.open(path_str.as_str());
            }
        }
    }));

    lua.execute::<()>(include_str!("editor.lua"))?;

    Ok(())
}
