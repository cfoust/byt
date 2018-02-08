// EXTERNS

// LIBRARY INCLUDES
use hlua;
use hlua::Lua;

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::Editor;

pub fn init_lua(editor : &mut Editor) {
    let mut lua = Lua::new();
    lua.openlibs();

    let editor_addr   = editor as *mut Editor as usize;
    println!("address before: {}", editor_addr);
    let editor_top    = editor_addr >> 32;
    let editor_bottom = editor_addr as u32 as usize;
    assert_eq!(editor_addr, (editor_top << 32) | editor_bottom);

    lua.set("__editor_addr_top", editor_top as u32);
    lua.set("__editor_addr_bottom", editor_bottom as u32);

    lua.execute::<()>("print(__editor_addr_top)").unwrap();
    lua.execute::<()>("print(__editor_addr_bottom)").unwrap();

    let restored_top : u32    = lua.get("__editor_addr_top").unwrap();
    let restored_bottom : u32 = lua.get("__editor_addr_bottom").unwrap();
    let restored_addr = ((restored_top as u64) << 32) | (restored_bottom as u64);
    println!("address after: {}", restored_addr);

    unsafe {
        let mut other_ptr = restored_addr as *mut Editor;
        let mut other_editor = &mut *other_ptr;
        other_editor.open("testfiles/no_line_ending.txt");
    }

    assert_eq!(editor.files.len(), 1);

    ::std::process::exit(0);
}
