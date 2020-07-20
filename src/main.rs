#[macro_use]
extern crate serde_derive;

mod html;
mod cfg;
mod md;

use crate::cfg::{HeaderLink, SiteConfig};
use clap::{Arg, App};
use toml;

use std::fs;

fn convert_file(target_name: &String, site_cfg: &SiteConfig) {

}

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

    let toml_string: String = fs::read_to_string("sss-config.toml").expect("Failed to open sss-config.toml");
    let toml_cfg: cfg::SiteConfig = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.fill_empty();

    let start_time = std::time::Instant::now();

    let paths = fs::read_dir("posts/").unwrap();
    for p in paths {
        let p = format!("{}", p.unwrap().path().display());
        //println!("{}", p.unwrap().path().display());

        let stream = md::parse_md_file(&p);
        let stream = match stream {
            Ok(s) => s,
            _ => panic!("Failed to obtain stream")
        };

        let mut target_name: String = p.chars().take_while(|x| *x != '.').collect();
        target_name.push_str(".html");
        println!("{}", target_name);

        match html::stream_to_html(stream, &target_name, &toml_cfg) {
            Ok(_) => (),
            Err(_) => println!("Failed to parse stream into HTML.")
        };
    }
    let duration = start_time.elapsed();
    println!("Site generation took {:?}", duration);
}
