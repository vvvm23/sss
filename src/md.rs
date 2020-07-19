/// This file handles the conversion from md to a vector of enums that describes
/// the various components of a markdown file. This will not be a comprehensive
/// representation of a markdown file, but will be sufficient for my use case.
///
/// In particular, I will begin by handlings:
///     - Headings using #, both enclosed by another # or not
///     - Paragraphs, a sequence of lines delimited by an empty line
///     - Images
///     - Hyperlinks
///     - Code blocks, (indented code blocks)
///

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub enum MDComponent {
    Heading(u8, String),
    Paragraph(String),
    Image(String),
    Hyperlink(String),
    CodeBlock(String),
}


pub fn parse_md_file(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
