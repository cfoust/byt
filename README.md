# byt

[![Build
Status](https://travis-ci.org/cfoust/byt.svg?branch=master)](https://travis-ci.org/cfoust/byt)
[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

byt is a flexibly ergonomic text editor for the terminal. Its primary goals are
efficiency and extensibility. It renders in full true color on supported
terminals and has a goal of being a command-line editor for the modern age.

What does that mean? You might feel that `vim` and `emacs` have the market
cornered on command-line editors. As someone who is a dedicated `vim` user, I
will be the first one to tell you its fatal flaw: bindings are only additive.
You cannot replace them. The basic movement keys cannot be changed or moved to
other keys. In addition, plugins have to do hacky work with text buffers to
make interfaces.

byt solves this problem by making plugins a first class citizen. They can have
arbitrary state machines for key bindings, receive hooks from the editor, and
render whatever they so desire when given certain events. With byt, the
configuration of your text editor is only limited by your imagination.

Most text editors set you up with a set of keybindings, maybe give you a live
editing context right off the bat, and let you go at it. Without its base
plugins, byt does not do that. All of the editor's basic functionality is
really comprised of a set of mutators. For now, that's just `vym`, byt's vim
emulation mode. byt aims for complete customizability of the development
experience when it comes to text by allowing you to replace everything if you
so choose.

The editor is very much a work in progress and currently is not even close to
being functional. See ROADMAP.md for a deeper understanding of where things are
and when, if at all, byt will be usable for general text editing.
