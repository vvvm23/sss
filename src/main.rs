mod config;
use clap::{Arg, App};
use toml;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct HeaderLink {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteConfig {
    title: Option<String>,
    header_links: Option<Vec<HeaderLink>>,
    index_path: Option<String>,
    style_path: Option<String>,
    page_dir: Option<String>,
    pub_dir: Option<String>,
}

impl SiteConfig {
    fn fill_empty(&mut self) -> &mut Self {
        // Populate missing .toml entries with some defaults
        if let None = self.title { self.title = Some("Default Site Title".to_string()) }
        if let None = self.header_links { self.header_links = Some(Vec::new()) }
        if let None = self.index_path { self.index_path = Some("pages/index.md".to_string()) }
        if let None = self.style_path { self.style_path = Some("styles/style.css".to_string()) }
        if let None = self.page_dir { self.page_dir = Some("pages/".to_string()) }
        if let None = self.pub_dir { self.pub_dir = Some("public/".to_string()) }
        self
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
    let mut site_cfg: SiteConfig = toml::from_str(&site_cfg_raw).unwrap();
    let site_cfg = site_cfg.fill_empty();

    println!("{:#?}", site_cfg);
}
