For now this is more of a to-do list but it generally defines the direction the
project is heading in terms of features. Bugs exist outside of this timeline.

- [x] Create the editor's buffer
  - [x] Implement a piece table
  - [x] Allow for undos and redos that properly restore the state of the piece
    table
  - [x] Record timestamps for every edit
  - [ ] Allow for exploring edits over time
- [x] Establish a system of interpreting key presses. Want this to be as robust
  as possible, allowing for an arbitrary number of modes and keybindings.
  Default to basic vim-esque bindings.
- [x] Define a Renderable trait that can render a window given its boundaries
  and and a struct that wraps all of Termion's render operations. This is so
  each pane can only render within its bounds. Trait should have `render()` and
  `should_render()` functions, which are used to render if necessary after
  every event.
- [ ] Create a FileView that acts as a viewport into a PieceFile. Manages all
  aspects of:
  - [x] reading
  - [x] rendering
  - [ ] writing
  - [ ] creating files
- [x] All keybindings that correspond to an action really are calling a
  closure by name. Come up with the system of scoping (i.e pane specific,
  global, etc) and passing mutable editor state into the functions.
- [x] Give the aforementioned closures a way to store and retrieve arbitrary
  state.
- [x] Set up a system of mutations, which optionally attach to the pane or the
  global editing context and provide a combination of binding tables and
  functions.
- [x] Revisit the FileView's buffer operations and make them airtight. This
  might involve rewriting some portion of view-specific rendering code.
- [ ] Create a system of plugins, called mutators, which are defined in gluon.
  These mutators can do the following:
  - [ ] Define sets of string-identified actions that mutate both mutator state
    and some aspect of editor state. At their most basic level, actions are
    just functions in gluon.
  - [ ] Define arbitrary keybindings as state machines. Mutators can receive
    input and respond to it, but can optionally decide to swallow the key
    event.
  - [ ] Asynchronous operations, like executing shell commands and sending
    requests on the network.
- [ ] Implement `vym`, byt's vim emulation mode. The goal isn't to be exactly
  like vim, just to make keybindings that are familiar to vim users. Most of
  this stuff relies upon changes to the FileView so that vym can actually
  manipulate state.
  - [ ] All common movement keys.
    - [x] HJKL
  - [ ] Insert mode.
    - [x] User can input text.
    - [ ] (FileView) Respect indentation levels as necessary.
  - [ ] Mode indicator.
  - [ ] Command bar that appears when you type `:` and allows you to do some
    subset of vim's operations like saving and opening files.
  - [ ] Registers for copying and pasting.
  - [ ] Visual mode that lets you select text.
> Editor should be usable at this point for common editing tasks
- [ ] Rewrite rendering logic to avoid flickering. This involves storing a
  logical representation of the characters currently on the screen and only
  updating those that have changed.
- [ ] Make byt a fully-fledged command line program with usage information,
  arguments, and otherwise.
- [ ] Generate/write man pages.
- [ ] Pane system sort of akin to vim's. Restrict rendering to a particular
  area of the screen.
- [ ] Create a UIView that extends Renderable and lets you create menus,
  dialogs, and graphs. We're not implementing those UI controls ourselves, but
  the goal is to have a view that isn't just a file buffer. All of vim's panes
  are file buffers and that was a good model then, but I'm writing this from
  scratch.
- [ ] As the user works with byt, optionally collect usage information that can
  suggest more efficient keybindings.
