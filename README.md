# Simple Static Sites

A minimalist static site generator written in Rust.

Designed to contain only the necessary features for my use cases

## Installation

Build from source with command `cargo run --release` assuming that cargo is installed.

## Usage

TODO

## Supported Syntax

List of the supported syntax so far:

- Paragraphing with multiple source file lines being part of the same paragraph
- ATX Headings of any depth (ATX headers are # form)
- Code Blocks in 4 space indentation form
- Images in exclamation point form
- Inline bold, italics, code and hyperlinks

## Configuration 

### sss-config.toml

TODO 

### styles/

TODO

### templates/ 

TODO

## Things to do

- Commands to generate project directories (init, new, clean, etc)
- Auto generate post list on index.html (or, in general, auto generation of index.html)
- Post tagging
- More robust error handling and messages
- HTML templating
- Rework of styles
