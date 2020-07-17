mod html;
use clap::{Arg, App};
use toml;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct HeaderLink {
    name: Option<String>, // display name for link
    url: Option<String>, // actual target for link
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteConfig {
    title: Option<String>, // site title, will appear in header and in browser title
    header_links: Option<Vec<HeaderLink>>, // array of links in the header
    index_path: Option<String>, // path to index.md file that will be compiled and placed in $pub_dir/index.html
    style_path: Option<String>, // path to style.css file that was be copied to $pub_dir/styles/style.css
    page_dir: Option<String>, // path to directory containing markdown files for pages
    pub_dir: Option<String>, // path to output directory where resulting website is placed
}

impl SiteConfig {
    fn fill_empty(&mut self) -> &mut Self {
        // Populate missing .toml entries with some defaults
        // TODO: Use proper path constructors
        if let None = self.title { self.title = Some("Default Site Title".to_string()) } // default to "Default Site Title"
        if let None = self.header_links { self.header_links = Some(Vec::new()) } // defaults to empty vec
        if let None = self.page_dir { self.page_dir = Some("pages/".to_string()) } // defaults to pages/
        if let None = self.index_path { self.index_path = Some(format!("{}{}", // defaults to $pub_dir/index.md
           match &self.page_dir {
               Some(c) => c,
               None => panic!()
           },
           "index.md"
           )) }

        if let None = self.style_path { self.style_path = Some("styles/style.css".to_string()) } // defaults to styles/style.css
        if let None = self.pub_dir { self.pub_dir = Some("public/".to_string()) } // defaults to public/
        self
    }
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

    // Read in sss-config.toml and deserialise into struct
    let site_cfg_raw = std::fs::read_to_string("sss-config.toml")
        .expect("Failed to read file.");
    println!("{:?}", site_cfg_raw);
    let mut site_cfg: SiteConfig = toml::from_str(&site_cfg_raw).unwrap();
    let site_cfg = site_cfg.fill_empty();

    println!("{:#?}", site_cfg);

    println!("{}", html::generate_head("afmck.in".to_string(), "styles/style.css".to_string()));

}
