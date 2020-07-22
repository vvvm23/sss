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

/// Enum containing paragraph components
#[derive(Debug)]
pub enum PGComponent {
    Text(String), // Default type
    Bold(String), // ** **
    Italics(String), // * *
    Hyperlink(String, String), // (text, url)
    Code(String), // Inline code
}

pub enum Inline {
    Text,
    Bold,
    Italics,
    Code,
}

/// Enum containing all supports markdown components
#[derive(Debug)]
pub enum MDComponent {
    Heading(u8, String),
    Paragraph(Vec<PGComponent>),
    Image(String, String),
    CodeBlock(String),
    Empty,
}

/// Used for defining the current block for multi-line blocks
#[derive(Copy, Clone)]
enum Block {
    Code,
    Paragraph,
}

/// Interpret Chars as a heading and return MDComponent::Heading
fn parse_heading(text: &mut std::str::Chars) -> MDComponent {
    let mut depth: u8 = 1;
    while text.next() == Some('#') {
        depth += 1;
    }

    MDComponent::Heading(depth, text.take_while(|x| *x != '#').collect())
}

/// Interpret String as a code block and return MDComponent::CodeBlock
fn parse_code(text: &String) -> MDComponent {
    MDComponent::CodeBlock(text.to_string())
}

/// Interpret String as a paragraph and return MDComponent::Paragraph
fn parse_paragraph(text: &String) -> MDComponent {
    //MDComponent::Paragraph(text.to_string())
    
    let mut current_comp: Option<Inline> = None;
    let mut current_block: String = "".to_string();
    let mut pg_vec: Vec<PGComponent> = Vec::new();

    let mut text_chars = text.chars();

    for c in text_chars {
        match c {
            '*' => {
                match text_chars.next() {
                    Some('*') => { // Bold
                        let bold: String = text_chars.take_while(|x| *x != '*').skip(2).collect(); // Technically will not check for closing **, only *
                        pg_vec.push(PGComponent::Bold(bold));
                    },
                    Some(c) => { // Italics
                        let italics: String = format!("{}{}", c, text_chars.take_while(|x| *x != '*').skip(1).collect::<String>()); // bit wack
                        pg_vec.push(PGComponent::Italics(italics));
                    },
                    None => { // Something went wrong

                    },
                };

            },
            '[' => {},
            ']' => {},
            '`' => {},
            _ => {
                if let Some(pc) = current_comp {
                    current_block.push(c);
                } else {
                    current_comp = Some(Inline::Text);
                    current_block.push(c);
                }
            },
        }
    }

    MDComponent::Paragraph(vec![])
}

/// Accepts path to markdown file and returns Vec<MDComponent> representing the file
pub fn parse_md_file(path: &str) -> std::io::Result<Vec<MDComponent>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    let mut md_vec: Vec<MDComponent> = Vec::new(); // Initialise stream to empty vec
    let mut block: String = "".to_string(); // Initialise current block to empty 
    let mut current_block: Option<Block> = None; // Set current block to None

    for (_, l) in f.lines().enumerate() {
        let line = l.unwrap().to_string(); 
        let mut line_chars = line.chars();
        
        let c = line_chars.next();

        let md_c = match c {
            // A bit dirty..
            Some('#') => { // Found a heading
                let md_cc = match current_block {
                    Some(Block::Paragraph) => Some(parse_paragraph(&block)),
                    Some(Block::Code) => Some(parse_code(&block)),
                    None => None,
                };

                if let Some(b) = md_cc {
                    md_vec.push(b);
                    block = "".to_string();
                    current_block = None;
                }

                Some(parse_heading(&mut line_chars))
            },
            Some(' ') => { // Potentially found a code block
                if line_chars.take(3).collect::<String>() == "   " {
                    let md_cc = match current_block {
                        Some(Block::Code) => None,
                        Some(Block::Paragraph) => Some(parse_paragraph(&block)),
                        None => None,
                    };

                    if let Some(b) = md_cc {
                        md_vec.push(b);
                        block = "".to_string();
                    }

                    current_block = Some(Block::Code);
                    block.push_str(&line.chars().skip(4).collect::<String>());
                    block.push_str("\n");

                } else { 
                    let md_cc = match current_block {
                        Some(Block::Code) => Some(parse_code(&block)),
                        Some(Block::Paragraph) => None,
                        None => None,
                    };

                    if let Some(b) = md_cc {
                        md_vec.push(b);
                        block = "".to_string();
                    }

                    current_block = Some(Block::Paragraph);
                    block.push_str(&line);
                    block.push_str(" ");
                }
                None
            },
            Some('!') => { // Found an image
                let alt_text: String = line_chars.skip_while(|x| *x != '[').skip(1).take_while(|x| *x != ']').collect();
                let url: String = line.chars().skip_while(|x| *x != '(').skip(1).take_while(|x| *x != ')').collect();
                Some(MDComponent::Image(alt_text, url))
            }
            None => {
                let md_cc = match current_block {
                    Some(Block::Paragraph) => Some(parse_paragraph(&block)),
                    Some(Block::Code) => Some(parse_code(&block)),
                    None => None,
                };

                if let Some(c) = md_cc {
                    md_vec.push(c);
                }

                block = "".to_string();
                current_block = None;

                None
            },
            _ => { // Found something else, interpret as paragraph
                block.push_str(&line);
                block.push_str(" ");
                current_block = Some(Block::Paragraph);
                None
            }
        };

        if let Some(c) = md_c {
            md_vec.push(c);
            block = "".to_string(); //
        }

    }

    // Add final block
    let md_cc = match current_block {
        Some(Block::Paragraph) => Some(parse_paragraph(&block)),
        Some(Block::Code) => Some(parse_code(&block)),
        None => None,
    };

    if let Some(b) = md_cc {
        md_vec.push(b);
    }

    Ok(md_vec)
}
