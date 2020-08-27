#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderLink {
    pub name: Option<String>, // display name for link
    pub url: Option<String>, // actual target for link
}

pub struct SiteConfig {
    pub title: String, // site title, will appear in header and in browser title
    pub header_links: Vec<HeaderLink>, // array of links in the header
    pub index_path: String, // path to index.md file that will be compiled and placed in $pub_dir/index.html
    pub style_path: String, // path to style.css file that was be copied to $pub_dir/styles/style.css
    pub page_dir: String, // path to directory containing markdown files for pages
    pub pub_dir: String, // path to output directory where resulting website is placed
    pub fonts_dir: String, // path to directory containing website fonts
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfigToml {
    pub title: Option<String>, // site title, will appear in header and in browser title
    pub header_links: Option<Vec<HeaderLink>>, // array of links in the header
    pub index_path: Option<String>, // path to index.md file that will be compiled and placed in $pub_dir/index.html
    pub style_path: Option<String>, // path to style.css file that was be copied to $pub_dir/styles/style.css
    pub page_dir: Option<String>, // path to directory containing markdown files for pages
    pub pub_dir: Option<String>, // path to output directory where resulting website is placed
    pub fonts_dir: Option<String>, // path to directory containing website fonts
}


impl SiteConfigToml {
    pub fn build_cfg(self) -> SiteConfig {
        let mut cfg = SiteConfig::new();
        if let Some(t) = self.title { cfg.title = t };
        if let Some(l) = self.header_links { cfg.header_links = l };
        if let Some(p) = self.page_dir { cfg.page_dir = p };
        if let Some(i) = self.index_path { cfg.index_path = i };
        if let Some(s) = self.style_path { cfg.style_path = s };
        if let Some(p) = self.pub_dir { cfg.pub_dir = p };
        if let Some(f) = self.fonts_dir { cfg.fonts_dir = f };

        cfg
    }
}

impl SiteConfig {
    pub fn new() -> Self {
        Self {
            title: "Default Site Title".to_string(),
            header_links: Vec::new(),
            page_dir: "posts/".to_string(),
            index_path: "posts/index.md".to_string(),
            style_path: "styles/style.css".to_string(),
            pub_dir: "public/".to_string(),
            fonts_dir: "fonts/".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: Option<String>,
    pub url: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostConfig {
    pub posts: Option<Vec<Post>>
}
