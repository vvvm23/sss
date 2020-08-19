#[macro_use]
extern crate serde_derive;

mod html;
mod cfg;
mod md;

use crate::cfg::{HeaderLink, SiteConfig};
use clap::{Arg, App};
use toml;

use std::fs;
use std::io::Write;

fn convert_file(source_name: &String, target_name: &String, site_cfg: &SiteConfig) {
    let stream = md::parse_md_file(&source_name);
    let stream = match stream {
        Ok(s) => s,
        _ => panic!("Failed to obtain stream")
    };

    match html::stream_to_html(stream, &target_name, &site_cfg) {
        Ok(_) => (),
        Err(_) => panic!("Failed to parse stream into HTML.")
    };
}

fn p_create_dir(path: String) {
    match fs::create_dir(path) {
        Ok(_) => (),
        Err(_) => panic!("Failed to create directory.."),
    };
}

fn new(project_name: String) {
    print!("Creating new project.. ");

    if let Err(_) = fs::create_dir(format!("./{}", project_name)) {
        println!("\nProject Directory already exists! Cancelling..\n");
        return
    }

    let f_cfg = fs::File::create(format!("./{}/{}", project_name, "sss-config.toml"));
    match f_cfg {
        Ok(f) => (),
        Err(_) => panic!("Failed to create file"),
    };

    let p_cfg = fs::File::create(format!("./{}/{}", project_name, "posts.toml"));
    match p_cfg {
        Ok(f) => (),
        Err(_) => panic!("Failed to create file"),
    };

    p_create_dir(format!("./{}/{}", project_name, "posts"));
        let f_index = fs::File::create(format!("./{}/{}", project_name, "posts/index.md"));
        match f_index {
            Ok(f) => (),
            Err(_) => panic!("Failed to create file"),
        };

    p_create_dir(format!("./{}/{}", project_name, "imgs"));

    p_create_dir(format!("./{}/{}", project_name, "styles"));
        let f_styles = fs::File::create(format!("./{}/{}", project_name, "styles/style.css"));
        match f_styles {
            Ok(f) => (),
            Err(_) => panic!("Failed to create file"),
        };

    p_create_dir(format!("./{}/{}", project_name, "fonts"));

    p_create_dir(format!("./{}/{}", project_name, "public"));
        p_create_dir(format!("./{}/{}", project_name, "public/posts"));
        p_create_dir(format!("./{}/{}", project_name, "public/fonts"));
        p_create_dir(format!("./{}/{}", project_name, "public/imgs"));
        p_create_dir(format!("./{}/{}", project_name, "public/styles"));

    println!("Done.\n");
}

fn clean() {
    print!("Cleaning public directory.. ");
    let toml_string: String = fs::read_to_string("sss-config.toml").expect("Failed to open sss-config.toml");
    let toml_cfg: cfg::SiteConfig = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.fill_empty();

    let pub_dir = &toml_cfg.pub_dir.unwrap();

    let files = fs::read_dir(pub_dir);
    if let Err(_) = files {
        println!("Failed to find public directory");
        return;
    }

    let files = fs::read_dir(format!("{}/fonts", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path());
    }

    let files = fs::read_dir(format!("{}/posts", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path());
    }

    let files = fs::read_dir(format!("{}/imgs", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path());
    }

    let files = fs::read_dir(format!("{}/styles", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path());
    }

    std::fs::remove_file(format!("{}/index.html", pub_dir));

    println!("Done.\n");
}


fn build() {
    print!("Building site into public directory.. ");

    let toml_string: String = fs::read_to_string("sss-config.toml").expect("Failed to open sss-config.toml");
    let toml_cfg: cfg::SiteConfig = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.fill_empty();

    let posts_string: String = fs::read_to_string("posts.toml").expect("Failed to open posts.toml");
    let posts_cfg: cfg::PostConfig = toml::from_str(&posts_string).unwrap();

    let index_path = match &toml_cfg.index_path {
        Some(p) => p,
        None => panic!("Missing index path!")
    };

    let posts_dir = match &toml_cfg.page_dir {
        Some(p) => p,
        None => panic!("Missing posts directory path!")
    };

    let style_path = match &toml_cfg.style_path {
        Some(p) => p,
        None => panic!("Missing style path!"),
    };

    let pub_dir = match &toml_cfg.pub_dir {
        Some(p) => p,
        None => panic!("Missing public directory path!")
    };

    let font_dir = match &toml_cfg.fonts_dir {
        Some(p) => p,
        None => panic!("Missing fonts directory path!")
    };

    let font_files = std::fs::read_dir(font_dir);
    for f in font_files.unwrap() {
        let f = f.unwrap().path();
        let f = f.to_str();
        let f = match f {
            Some(s) => s,
            None => panic!()
        };

        fs::copy(f, format!("{}/{}", pub_dir, f));
    }

    let start_time = std::time::Instant::now();

    // Allow no styles/style.css
    match std::fs::copy(&style_path, format!("{}/{}", pub_dir, style_path)) {
        Ok(_) => (),
        Err(_) => println!("Failed to copy style file")
    }


    let posts = match posts_cfg.posts {
        Some(p) => p,
        None => vec![]
    };

    //convert_file(index_path, &"index.html".to_string(), &toml_cfg);
    let index_stream = md::parse_md_file(&index_path);
    let mut index_stream = match index_stream {
        Ok(s) => s,
        _ => panic!("Failed to obtain stream")
    };

    index_stream.push(md::MDComponent::Heading(3, "Recent Posts".to_string()));

    let paths = fs::read_dir(posts_dir).unwrap();
    //for p in paths {
        //let p = format!("{}", p.unwrap().path().display());
        //let mut target_name: String = p.chars().take_while(|x| *x != '.').collect();
        //target_name.push_str(".html");
        //let tp = target_name;
        //convert_file(&p, &tp, &toml_cfg);
    //}
    
    for p in posts {
        let url = p.url.unwrap();
        let title = p.title.unwrap();

        let mut tp: String = url.chars().take_while(|x| *x != '.').collect();
        tp.push_str(".html");
        convert_file(&url, &tp, &toml_cfg);

        let link_block = md::MDComponent::Paragraph(vec![md::PGComponent::Hyperlink(title, tp)]);
        index_stream.push(link_block);

    }
    match html::stream_to_html(index_stream, &"index.html".to_string(), &toml_cfg) {
        Ok(_) => (),
        Err(_) => println!("Failed to parse stream into HTML.")
    };
    let duration = start_time.elapsed();
    println!("Done.");
    println!("Site generation took {:?}", duration);
}

fn add(title: &str, path: &str) {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("posts.toml");

    let mut file = match file {
        Ok(f) => f,
        Err(_) => panic!("Failed to open posts.toml")
    };

    writeln!(file, "\n[[posts]]");
    writeln!(file, "{}", format!("title = \"{}\"", title));
    writeln!(file, "{}", format!("url = \"posts/{}.md\"", path));

    let f_post = fs::File::create(format!("./posts/{}.md", path));
    match f_post {
        Ok(_) => (),
        Err(_) => panic!("Failed to create post file!")
    };
}

fn main() {
    // Define command line arguments
    let matches = App::new("Simple Static Sites")
        .version("0.1-alpha")
        .author("Alexander McKinney <alexander.f.mckinney@durham.ac.uk>")
        .about("Generates a website from a collection of markdown files")
        .subcommand(App::new("build")
            .about("Commands to generate a website from markdown files")
            .arg(Arg::with_name("clean")
                .short("c")
                .long("clean")
                .help("Clean before building")
                .takes_value(false)))

        .subcommand(App::new("new")
            .about("Commands to create a new project.")
            .arg(Arg::with_name("DIRECTORY")
                .help("Give the project directory name")
                .required(true)
                .index(1)))

        .subcommand(App::new("clean")
            .about("Clean public/ directory"))

        .subcommand(App::new("add")
            .about("Add a new post to the site")
            .arg(Arg::with_name("TITLE")
                 .help("Human-readable title of the post.")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("FILE")
                 .help("File name of the new post.")
                 .required(true)
                 .index(2)))

        .get_matches();

    match matches.subcommand() {
        ("new", Some(sc_m)) => {
            match sc_m.value_of("DIRECTORY") {
                Some(d) => new(d.to_string()),
                None => println!("No project name specified"),
            };
        },
        ("build", Some(sc_m)) => {
            match sc_m.is_present("clean") {
                true => clean(),
                false => (),
            };
            build();
        },
        ("clean", Some(sc_m)) => clean(),
        ("add", Some(sc_m)) => {
            match (sc_m.value_of("TITLE"), sc_m.value_of("FILE")) {
                (None, _) => println!("Missing argument"),
                (_, None) => println!("Missing argument"),
                (Some(t), Some(f)) => add(t, f)
            }
        }
        _ => println!("No subcommand specified. Please specify a subcommand")
    };

}
