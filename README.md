# Simple Static Sites

A minimalist static site generator written in Rust.

Designed to contain only the necessary features for my use cases

## Installation

Build from source with command `cargo run --release` assuming that cargo is installed.

To install the program, run the command `cargo install --path /path/to/project/root`.
Ensure you have added `$XDG_HOME/.cargo/bin` to your PATH.

## Usage

To create a new *sss* project, run the command `sss new PROJECT_NAME`.
This will create a new directory and populate it with a basic directory structure.

> Currently, the program is quite sensitive to this directory structure, so for now 
> keep with it. Redefining directory locations in `sss-config.toml` is at your own
> peril.

To create a new post, run the command `sss add POST_NAME POST_URL` within the root directory.
This will create a new file at `posts/POST_URL` and add a new entry to `post.toml`.
You can then edit the file to create a post.

To build the website, run the command `sss build [-c]` within the root directory.
The `-c` flag will clean the `public/` directory before building.
This command will compile all site markdown files into a static site which will 
be placed in `public/`. Any resource files will be copied too.

To clean the `public/` directory, run the command `sss clean` within the root directory.

> This will irreversibly delete any files in `public/`. Use at your own peril.

Help messages can be accessed by running `sss [--help] | [-h]`

## Supported Syntax

List of the supported syntax so far:

- Paragraphing with multiple source file lines being part of the same paragraph
- ATX Headings of any depth (ATX headers are # form)
- Code Blocks in 4 space indentation form
- Images in exclamation point form
- Inline bold, italics, code and hyperlinks
- Block Quotes (no inner md support yet though)
- ~Inject HTML by simply writing it normally (a "feature" I guess)~ This is apparently correct behaviour

## Features

- Syntax highlighting of code blocks using hightlight.js
- Inline and Display Math mode using MathJax

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
- Clean up some remaining bad error handling
- Add more context to handled errors so users know how to fix
