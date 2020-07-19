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

#[derive(Debug)]
pub enum MDComponent {
    Heading(u8, String),
    Paragraph(String),
    Image(String),
    Hyperlink(String),
    CodeBlock(String),
    Empty,
}

fn parse_heading(text: &mut std::str::Chars) -> MDComponent {
    let mut depth: u8 = 1;
    while text.next() == Some('#') {
        depth += 1;
    }

    MDComponent::Heading(depth, text.collect())
}

fn parse_code(text: String) -> MDComponent {
    MDComponent::CodeBlock(text)
}

fn parse_paragraph(text: String) -> MDComponent {
    MDComponent::Paragraph(text)
}

/// Accepts path to markdown file and returns Vec<MDComponent> representing the file
pub fn parse_md_file(path: &str) -> std::io::Result<Vec<MDComponent>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    let mut md_vec: Vec<MDComponent> = Vec::new();

    for (i, l) in f.lines().enumerate() {
        let line = l.unwrap().to_string();
        let mut line_chars = line.chars();
        
        let c = line_chars.next();

        let md_c = match c {
            // A bit dirty..
            Some('#') => parse_heading(&mut line_chars),
            Some(' ') => parse_code(line_chars.skip(3).take_while(|_| true).collect::<String>()), 
            None => MDComponent::Empty,
            _ => parse_paragraph(line),
        };
        md_vec.push(md_c);

    }

    Ok(md_vec)
}
