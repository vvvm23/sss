#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderLink {
    pub name: Option<String>, // display name for link
    pub url: Option<String>, // actual target for link
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: Option<String>, // site title, will appear in header and in browser title
    pub header_links: Option<Vec<HeaderLink>>, // array of links in the header
    pub index_path: Option<String>, // path to index.md file that will be compiled and placed in $pub_dir/index.html
    pub style_path: Option<String>, // path to style.css file that was be copied to $pub_dir/styles/style.css
    pub page_dir: Option<String>, // path to directory containing markdown files for pages
    pub pub_dir: Option<String>, // path to output directory where resulting website is placed
}

// TODO: A new struct with no Option in the fields. fill_empty will return this
//       This will help later so we don't need to unpack lots of Option
impl SiteConfig {
    pub fn fill_empty(mut self) -> Self {
        // Populate missing .toml entries with some defaults
        // TODO: Use proper path constructors
        if let None = self.title { self.title = Some("Default Site Title".to_string()) } // default to "Default Site Title"
        if let None = self.header_links { self.header_links = Some(Vec::new()) } // defaults to empty vec
        if let None = self.page_dir { self.page_dir = Some("posts/".to_string()) } // defaults to pages/
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

