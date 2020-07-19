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

    // Read in sss-config.toml and deserialise into struct
    
    //let site_cfg_raw = std::fs::read_to_string("sss-config.toml")
        //.expect("Failed to read file.");
    //println!("{:?}", site_cfg_raw);
    //let mut site_cfg: SiteConfig = toml::from_str(&site_cfg_raw).unwrap();
    //let site_cfg = site_cfg.fill_empty();

    //println!("{:#?}", site_cfg);

    //println!("{}", html::generate_head("afmck.in".to_string(), "styles/style.css".to_string()));
    //println!("{}", html::generate_header("afmck.in".to_string(), vec![
        //HeaderLink { name: Some("GitHub".to_string()), url: Some("https://github.com/vvvm23".to_string()) },
        //HeaderLink { name: Some("Twitter".to_string()), url: Some("https://twitter.com/alexfmckinney".to_string()) }
    //]));
    //println!("{}", html::generate_content("posts/index.md".to_string()));
    //
    
    println!("{:?}", md::parse_md_file("./test.md"));

}
