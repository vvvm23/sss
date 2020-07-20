#[macro_use]
extern crate serde_derive;

mod html;
mod cfg;
mod md;

use crate::cfg::{HeaderLink, SiteConfig};
use clap::{Arg, App};
use toml;

fn main() {
    // Define command line arguments
    let matches = App::new("Simple Static Sites")
        .version("0.1-alpha")
        .author("Alexander McKinney <alexander.f.mckinney@durham.ac.uk>")
        .about("Generates a website from a collection of markdown files")
        .arg(Arg::with_name("test")
            .short("t")
            .long("test")
            .help("A test argument for learning purposes.")
            .takes_value(true)
        )
        .get_matches();

    // Example of argument evaluation
    match matches.value_of("test") {
        Some(v) => println!("{}", v),
        None => println!("No argument.")
    }

    let toml_string: String = std::fs::read_to_string("sss-config.toml").expect("Failed to open sss-config.toml");
    let mut toml_cfg: cfg::SiteConfig = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.fill_empty();

    let stream = md::parse_md_file("./test.md");
    let stream = match stream {
        Ok(s) => s,
        _ => panic!("Failed to obtain stream")
    };

    match html::stream_to_html(stream, toml_cfg) {
        Ok(_) => (),
        Err(_) => println!("Failed to parse stream into HTML.")
    };

}
