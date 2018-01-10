For now this is more of a to-do list but it generally defines the direction the project is heading in terms of features. Bugs exist outside of this timeline.

- [X] Create the editor's buffer
  - [X] Implement a piece table
  - [X] Allow for undos and redos that properly restore the state of the piece
    table
  - [X] Record timestamps for every edit
  - [ ] Allow for exploring edits over time
- [X] Establish a system of interpreting key presses. Want this to be as robust
  as possible, allowing for an arbitrary number of modes and keybindings.
  Default to basic vim-esque bindings.
- [X] Define a Renderable trait that can render a window given its boundaries
  and and a struct that wraps all of Termion's render operations. This is so
  each pane can only render within its bounds. Trait should have `render()` and
  `should_render()` functions, which are used to render if necessary after
  every event.
- [ ] Create a FileView that acts as a viewport into a PieceFile. Manages all
  aspects of [X] reading, [ ] writing, and [ ] creating files.
- [X] All keybindings that correspond to an action really are calling a
  closure by name. Come up with the system of scoping (i.e pane specific,
  global, etc) and passing mutable editor state into the functions.
- [X] Give the aforementioned closures a way to store and retrieve arbitrary
  state.
- [X] Set up a system of mutations, which optionally attach to the pane or the
  global editing context and provide a combination of binding tables and
  functions.
- [X] Revisit the FileView's buffer operations and make them airtight. This
  might involve rewriting some portion of view-specific rendering code.
- [ ] Refine mutation system to be a bit more logical and easier to understand.
  Mutators should be able to affect the flow of any part of the editor.
- [ ] Include gluon for editor extensibility. `gluon` plugins should be able to
  do anything a Rust-defined plugin can. Also, gluon's functions should have
    access to all of the editor state that Rust-defined functions would.
- [ ] Implement `vym`, byt's vim emulation mode. The goal isn't to be exactly
  like vim, just to make keybindings that are familiar to vim users.
  - [ ] All common movement keys.
  - [X] Insert mode.
  - [ ] Mode indicator.
  - [ ] Command bar that appears when you type `:` and allows you to do some
    subset of vim's operations like saving and opening files.
  - [ ] Registers for copying and pasting.
  - [ ] Visual mode.
- [ ] Rewrite rendering logic to avoid flickering. This involves storing a
  logical representation of the characters currently on the screen and only
  updating those that have changed.
- [ ] Pane system sort of akin to vim's. Restrict rendering to a particular
  area of the screen.
- [ ] Create a UIView that extends Renderable and lets you create menus,
  dialogs, and graphs. We're not implementing those UI controls ourselves, but
  the goal is to have a view that isn't just a file buffer. All of vim's panes
  are file buffers and that was a good model then, but I'm writing this from
  scratch.
- [ ] As the user works with byt, optionally collect usage information that can
  suggest more efficient keybindings.

