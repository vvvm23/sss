use crate::cfg::{HeaderLink, SiteConfig};

pub fn generate_head(title: String, style_path: String) -> String {
    // TODO: Optional highlight.js
    format!("<head>\
        <title>{}</title>\
        <meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\" />\
        <link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\
        <link rel=\"stylesheet\" href=\"//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/styles/default.min.css\">\
        <script src=\"//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/highlight.min.js\"></script>\
        <script>hljs.initHighlightingOnLoad();</script>\
        </head>\
    ", title, style_path)
}

fn generate_header_links(links: Vec<HeaderLink>) -> String {
    let mut header = "".to_string();
    for l in links {
        let name = match l.name {
            Some(n) => n,
            None => "missing-name".to_string()
        };
        let url = match l.url {
            Some(u) => u,
            None => "missing-url".to_string()
        };

        header.push_str(&format!("<a href=\"{}\">{}</a>", url, name));
    }

    header
}

fn generate_content_body(post_path: String) -> String {
    "epic".to_string()
}

pub fn generate_content(post_path: String) -> String {
    format!("<div class=\"content\">\
    {}\
    </div>\
    ", generate_content_body(post_path))
}

pub fn generate_header(title: String, links: Vec<HeaderLink>) -> String {
    format!("<div class=\"header\">\
        <span class=\"sitetitle\">{}</span>\
        {}\
        </div>\
    ", title, generate_header_links(links))
}
