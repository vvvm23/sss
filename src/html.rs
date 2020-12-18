/// This file handles the conversion from the markdown "stream", created by
/// the md library file, to a minimal HTML file which will be placed in the
/// public directory.
///
/// This library will handle all the markdown features that I defined in
/// md.rs.
///
/// This is still a WIP. panic(s) should be replaced with proper logging
///
use crate::cfg::{HeaderLink, SiteConfig};
use crate::md::{MDComponent, PGComponent};
use std::fs::File;
use std::io::{BufWriter, Write};

/// Generate HTML head block
pub fn generate_head(title: &String, style_path: &String) -> String {
    // TODO: Optional highlight.js
    // TODO: If highlight.js is wanted, should obtain a local copy so it can work offline.
    format!("<head>\
        <title>{}</title>\
        <meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\" />\
        <link rel=\"stylesheet\" type=\"text/css\" href=\"/{}\">\
<script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>\
<script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>\
        <link rel=\"stylesheet\" href=\"//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/styles/default.min.css\">\
        <script src=\"//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/highlight.min.js\"></script>\
        <script>hljs.initHighlightingOnLoad();</script>\
        </head>\
    ", title, style_path)
}

/// Generate HTML links in header
fn generate_header_links(links: &Vec<HeaderLink>) -> String {
    let mut header = "".to_string();
    for l in links {
        let name = match &l.name {
            Some(n) => n,
            None => panic!("HeaderLink had missing name."),
        };
        let url = match &l.url {
            Some(u) => u,
            None => panic!("HeaderLink had missing URL."),
        };

        header.push_str(&format!("<a href=\"{}\">{}</a>", url, name));
    }

    header
}

/// Generate HTML for header in body
pub fn generate_header(title: &String, links: &Vec<HeaderLink>) -> String {
    format!(
        "<div class=\"header\">\
        <a href=\"/\" class=\"sitetitle\">{}</span>\
        {}\
        </div>\
    ",
        title,
        generate_header_links(links)
    )
}

/// Generate HTML from paragraph stream
pub fn generate_paragraph(stream: Vec<PGComponent>) -> String {
    let mut para_str = "<p>".to_string();
    for pc in stream {
        // For each PGComponent, generate corresponding HTML and append to string
        let pg_str: String = match pc {
            PGComponent::Text(t) => t,
            PGComponent::Bold(t) => format!("<b>{}</b>", t),
            PGComponent::Italics(t) => format!("<i>{}</i>", t),
            PGComponent::Code(t) => format!("<code>{}</code>", t),
            PGComponent::Hyperlink(t, u) => format!("<a href=\"{}\">{}</a>", u, t),
            PGComponent::Math(t) => format!("\\({}\\)", t),
        };
        para_str.push_str(&pg_str);
    }
    para_str.push_str("</p>");
    para_str
}

/// Takes a stream (Vec<MDComponent>) and a title and writes to HTML file
pub fn stream_to_html(
    stream: Vec<MDComponent>,
    path: &String,
    site_cfg: &SiteConfig,
) -> std::io::Result<()> {
    let title = &site_cfg.title;
    let style_path = &site_cfg.style_path;
    let header_links = &site_cfg.header_links;
    let pub_dir = &site_cfg.pub_dir;

    let f = File::create(format!("{}/{}", pub_dir, path)).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write(" <!DOCTYPE html>".as_bytes())?;
    f.write("<html>".as_bytes())?;
    let head = generate_head(&title, &style_path);
    f.write(head.as_bytes())?;
    f.write("<body>".as_bytes())?;

    let header = generate_header(&title, header_links);
    f.write(header.as_bytes())?;

    f.write("<hr>".as_bytes())?;
    f.write("<div class=\"content\">".as_bytes())?;
    for mdc in stream {
        match mdc {
            MDComponent::Heading(d, t) => f.write(format!("<h{}>{}</h{}>", d, t, d).as_bytes())?,
            MDComponent::Paragraph(ps) => f.write(generate_paragraph(ps).as_bytes())?,
            MDComponent::Image(t, u) => {
                std::fs::copy(format!(".{}", &u), format!("{}/{}", pub_dir, &u))?;
                f.write(
                    format!(
                        "<figure><img src=\"{}\" alt=\"{}\"><figcaption>{}</figcaption></figure>",
                        u, t, t
                    )
                    .as_bytes(),
                )?
            }
            MDComponent::CodeBlock(t) => {
                f.write(format!("<pre><code>{}</code></pre>", t).as_bytes())?
            }
            MDComponent::Quote(t) => {
                f.write(format!("<blockquote>{}</blockquote>", t).as_bytes())?
            }
            MDComponent::Math(t) => {
                f.write(format!("$${}$$", t).as_bytes())?
            }
            //MDComponent::Empty => f.write("".as_bytes())?,
        };
    }
    f.write("</div>".as_bytes())?;
    f.write("</body>".as_bytes())?;
    f.write("</html>".as_bytes())?;

    Ok(())
}
