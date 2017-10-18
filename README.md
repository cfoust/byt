# byt

[![Build Status](https://travis-ci.org/cfoust/byt.svg?branch=master)](https://travis-ci.org/cfoust/byt)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Byt is a flexibly ergonomic text editor for the terminal. Its primary goals are
embedability, efficiency, and extensibility. Think `emacs` but taken a step
further.

The editor is very much a work in progress and currently is not even close to being
functional. The roadmap is as follows:
- [ ] Create the editor's buffer
  - [X] Implement a piece table
  - [ ] Allow for undos that properly restore the state of the piece table
  - [ ] Record timestamps for every edit to allow for exploring the file over
    time in the future
- [ ] Set up the rudiments of the editor itself. Should have no knowledge of rendering, just
      implements functionality over buffers.
- [ ] Include Lua for editor extensibility.
- [ ] Establish a system of interpreting key presses. Want this to be as robust as possible,
      allowing for an arbitrary number of modes and keybindings. Default to basic vim-esque
      bindings.
- [ ] Build the basic terminal renderer. In the future, we could render to a standalone
      window of some sort.
- [ ] As the user works with byt, optionally collect usage information that can suggest
      more efficient keybindings.
