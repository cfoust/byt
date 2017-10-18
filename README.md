# byt

[![Build Status](https://travis-ci.org/cfoust/byt.svg?branch=master)](https://travis-ci.org/cfoust/byt)

Byt is a flexibly ergonomic text editor for the terminal. It aims to be a vim for the modern age built with concurrency, requests, and configurability in mind.

I loved the world I found when I started playing around with vim and emacs. For me, they blew IDE's and other GUI text editors out of the water. I tweaked configurations as a tic and my `vimrc` blossomed into hundreds of lines over many files, even including a custom plugin just for me. There was something missing, though, in that I wanted to configure things that just couldn't be changed. The codebases of vim and emacs weren't approachable enough that I felt I could contribute the changes I wanted without stepping on toes. So why not write my own editor?

That's where byt came in. It's very much a work in progress, but the MVP goal is to give a similar default environment to vim's, but to allow you to change every aspect of how the editor works through Lua.

Coincidentally, all of byt will be written in vim. You have to start somewhere.
