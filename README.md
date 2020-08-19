# Simple Static Sites

A minimalist static site generator written in Rust.

Designed to contain only the necessary features for my use cases

## Installation

Build from source with command `cargo run --release` assuming that cargo is installed.

To install the program, run the command `cargo install --path /path/to/project/root`.
Ensure you have added `$XDG_HOME/.cargo/bin` to your PATH.

## Usage

TODO

## Supported Syntax

List of the supported syntax so far:

- Paragraphing with multiple source file lines being part of the same paragraph
- ATX Headings of any depth (ATX headers are # form)
- Code Blocks in 4 space indentation form
- Images in exclamation point form
- Inline bold, italics, code and hyperlinks
- Block Quotes (no inner md support yet though)
- Inject HTML by simply writing it normally (a "feature" I guess)

## Configuration 

### sss-config.toml

TODO 

### styles/

TODO

### templates/ 

TODO

## Things to do

- Post tagging
- HTML templating
- Rework of styles
- In general, more resistant to misuse and more error handling
