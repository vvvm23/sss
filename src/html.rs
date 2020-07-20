use crate::cfg::HeaderLink;
use crate::md::MDComponent;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Generate HTML <head> block
pub fn generate_head(title: &String, style_path: &String) -> String {
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

/// Generate HTML links in header 
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

/// Generate HTML for header in body
pub fn generate_header(title: &String, links: Vec<HeaderLink>) -> String {
    format!("<div class=\"header\">\
        <span class=\"sitetitle\">{}</span>\
        {}\
        </div>\
    ", title, generate_header_links(links))
}

/// Takes a stream (Vec<MDComponent>) and a title and writes to public/index.html
// TODO: parse title from file? MDComponent::Title 
// TODO: define output file based on input file (maintain directory structure)
pub fn stream_to_html(stream: Vec<MDComponent>, title: String) -> std::io::Result<()> {
    let f = File::create("public/index.html").expect("Unable to create file");
    let mut f = BufWriter::new(f);

    let style_path = "styles/style.css".to_string();
    let style_res = std::fs::copy(&style_path, format!("public/{}", &style_path));

    // Allow no styles/style.css
    match style_res {
        Ok(_) => (),
        Err(_) => println!("Failed to copy style file")
    }

    f.write("<html>".as_bytes())?;
    let head = generate_head(&title, &style_path);
    f.write(head.as_bytes())?;
    f.write("<body>".as_bytes())?;

    let header = generate_header(&title, vec![]);
    f.write(header.as_bytes())?;

    for mdc in stream {
        match mdc {
            MDComponent::Heading(d, t) => f.write(format!("<h{}>{}</h{}>", d, t, d).as_bytes())?,
            MDComponent::Paragraph(t) => f.write(format!("<p>{}</p>", t).as_bytes())?,
            MDComponent::Image(t, u) => {
                std::fs::copy(&u, format!("public/{}", &u))?;
                f.write(format!("<figure><img src=\"{}\" alt=\"{}\"><figcaption>{}</figcaption></figure>", u, t, t).as_bytes())?},
            MDComponent::CodeBlock(t) => f.write(format!("<pre><code>{}</code></pre>", t).as_bytes())?,
            MDComponent::Empty => f.write("".as_bytes())?,
            _ => f.write("".as_bytes())?
        };
    }

    f.write("</body>".as_bytes())?;
    f.write("</html>".as_bytes())?;

    Ok(())
}

