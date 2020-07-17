mod config;
use clap::{Arg, App};
use toml;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct SiteConfig {
    title: String,
    style_path: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        SiteConfig {
            title: "Default Site".to_string(),
            style_path: "style/style.css".to_string(),
        }
    }
}

fn main() {
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

    match matches.value_of("test") {
        Some(v) => println!("{}", v),
        None => println!("No argument.")
    }

    let site_cfg_raw = std::fs::read_to_string("sss-config.toml")
        .expect("Failed to read file.");
    println!("{:?}", site_cfg_raw);
    let site_cfg: SiteConfig = toml::from_str(&site_cfg_raw).unwrap();
    println!("{:?}", site_cfg);
}
