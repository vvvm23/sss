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

//pub fn generate_header(title: String, links: )
