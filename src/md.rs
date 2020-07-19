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

/// Accepts path to markdown file and returns Vec<MDComponent> representing the file
pub fn parse_md_file(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    for (i, line) in f.lines().enumerate() {
        let line = line.unwrap().to_string();
        let mut line_chars = line.chars();
        
        let c = line_chars.next();

        match c {
            Some('#') => println!("HEADER"),
            Some(' ') => println!("CODE"), // needs 4 spaces, not a tab
            None => println!("Empty Line"),
            _ => println!("Unknown."),
        }

    }

    Ok(())
}
