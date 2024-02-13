# Editor

A simple text editor written in Rust.

## Features
- Reads and renders a text file
- Has a cursor you can move around with the arrow keys
- Home & End work
- You can type (most of the time)
- Adjustable line-height (F1 / F2)
- No tests

## Roadmap
- Inserting and removing line breaks
- Save a file
- Scrolling
- Select text
- Copy, cut, & paste
- Find & replace
- Configurable keymaps
- Multiple tabs
- File tree
- Multiple cursors
- Connect to LSPs
    - Syntax highlighting
    - Code completion

## Running

### Linux

Make sure you have sdl2 libs installed (for ubuntu `libsdl2-dev` and `libsdl2-ttf-dev`) and then just `cargo run`, passing it the file you wish to open as an argument.

```
cargo run ./example.js
```
