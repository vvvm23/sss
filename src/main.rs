// TODO: Clean up remaining panic!(s) and replace with recoverable code.
#[macro_use]
extern crate serde_derive;

mod html;
mod cfg;
mod md;

use crate::cfg::SiteConfig;
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

fn new(project_name: String) -> Result<(), std::io::Error> {
    print!("Creating new project.. ");

    if let Err(_) = fs::create_dir(format!("./{}", project_name)) {
        println!("\nProject Directory already exists! Cancelling..\n");
        return Ok(())
    }

    fs::File::create(format!("./{}/{}", project_name, "sss-config.toml"))?;
    fs::File::create(format!("./{}/{}", project_name, "posts.toml"))?;

    fs::create_dir(format!("./{}/{}", project_name, "posts"))?;
        fs::File::create(format!("./{}/{}", project_name, "posts/index.md"))?;

    fs::create_dir(format!("./{}/{}", project_name, "imgs"))?;

    fs::create_dir(format!("./{}/{}", project_name, "styles"))?;
        fs::File::create(format!("./{}/{}", project_name, "styles/style.css"))?;

    fs::create_dir(format!("./{}/{}", project_name, "fonts"))?;

    fs::create_dir(format!("./{}/{}", project_name, "public"))?;
        fs::create_dir(format!("./{}/{}", project_name, "public/posts"))?;
        fs::create_dir(format!("./{}/{}", project_name, "public/fonts"))?;
        fs::create_dir(format!("./{}/{}", project_name, "public/imgs"))?;
        fs::create_dir(format!("./{}/{}", project_name, "public/styles"))?;

    println!("Done.\n");
    Ok(())
}

fn clean() -> Result<(), std::io::Error> {
    print!("Cleaning public directory.. ");
    let toml_string: String = fs::read_to_string("sss-config.toml")?;
    let toml_cfg: cfg::SiteConfigToml = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.build_cfg();

    let pub_dir = &toml_cfg.pub_dir;

    let files = fs::read_dir(pub_dir);
    if let Err(e) = files {
        println!("Failed to find public directory");
        return Err(e);
    }

    let files = fs::read_dir(format!("{}/fonts", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path())?;
    }

    let files = fs::read_dir(format!("{}/posts", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path())?;
    }

    let files = fs::read_dir(format!("{}/imgs", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path())?;
    }

    let files = fs::read_dir(format!("{}/styles", pub_dir));
    for f in files.unwrap() {
        let f = f.unwrap();
        std::fs::remove_file(f.path())?;
    }

    std::fs::remove_file(format!("{}/index.html", pub_dir))?;

    println!("Done.\n");
    Ok(())
}


fn build() -> Result<(), std::io::Error>
{
    print!("Building site into public directory.. ");

    let toml_string: String = fs::read_to_string("sss-config.toml")?;
    let toml_cfg: cfg::SiteConfigToml = toml::from_str(&toml_string).unwrap();
    let toml_cfg = toml_cfg.build_cfg();

    let posts_string: String = fs::read_to_string("posts.toml").expect("Failed to open posts.toml");
    let posts_cfg: cfg::PostConfig = toml::from_str(&posts_string).unwrap();

    let index_path = &toml_cfg.index_path;
    let style_path = &toml_cfg.style_path;
    let pub_dir = &toml_cfg.pub_dir;
    let font_dir = &toml_cfg.fonts_dir;

    let font_files = std::fs::read_dir(font_dir);
    for f in font_files.unwrap() {
        let f = f.unwrap().path();
        let f = match f.to_str() {
            Some(s) => s,
            None => panic!()
        };

        fs::copy(f, format!("{}/{}", pub_dir, f))?;
    }

    let start_time = std::time::Instant::now();

    // Allow no styles/style.css
    match std::fs::copy(&style_path, format!("{}/{}", pub_dir, style_path)) {
        Ok(_) => (),
        Err(_) => println!("No style.css found at {}", style_path)
    }

    // Allow no posts
    let posts = match posts_cfg.posts {
        Some(p) => p,
        None => vec![]
    };

    let posts: Vec<cfg::Post> = posts.into_iter().rev().collect();

    let index_stream = md::parse_md_file(&index_path);
    let mut index_stream = match index_stream {
        Ok(s) => s,
        _ => panic!("Failed to obtain stream")
    };

    index_stream.push(md::MDComponent::Heading(3, "Recent Posts".to_string()));

    for p in posts {
        let url = p.url.unwrap();
        let title = p.title.unwrap();

        let mut tp: String = url.chars().take_while(|x| *x != '.').collect();
        tp.push_str(".html");
        convert_file(&url, &tp, &toml_cfg);

        let link_block = md::MDComponent::Paragraph(vec![md::PGComponent::Hyperlink(title, tp)]);
        index_stream.push(link_block);

    }
    html::stream_to_html(index_stream, &"index.html".to_string(), &toml_cfg)?;
    //if let Err(e) = html::stream_to_html(index_stream, &"index.html".to_string(), &toml_cfg) {
        //return Err(e);
    //};
    let duration = start_time.elapsed();
    println!("Done.");
    println!("Site generation took {:?}", duration);

    Ok(())
}

fn add(title: &str, path: &str) -> Result<(), std::io::Error> {
    print!("Adding new post.. ");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("posts.toml")?;

    writeln!(file, "\n[[posts]]")?;
    writeln!(file, "{}", format!("title = \"{}\"", title))?;
    writeln!(file, "{}", format!("url = \"posts/{}.md\"", path))?;

    // TODO: If the file exists, maybe don't delete?
    // TODO: Or perhaps a separate command for the above
    fs::File::create(format!("./posts/{}.md", path))?;

    println!("Done.");
    println!("Created new post \"{}\"", title);

    Ok(())
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
            if let Some(d) = sc_m.value_of("DIRECTORY") {
                let r = new(d.to_string());
                if let Err(e) = r {
                    println!("\nAn error occurred whilst performing operation 'add'.");
                    println!("Check you have permission to create new directories and files in this location.");
                    println!("Also check the project name is valid.\n");
                    println!("The error was: ");
                    println!("{}\n", e);
                    println!("Project may be in inconsistent state. Please check for a new directory with the project name and delete it.");
                    return;
                }
            } else {
                println!("No project name specified.");
            }
        },
        ("build", Some(sc_m)) => {
            if sc_m.is_present("clean") {
                let r = clean();
                if let Err(e) = r {
                    println!("\nAn error occurred whilst performing operation 'clean'.");
                    println!("Check you are in the root of sss project directory and that the directory structure is valid.\n");
                    println!("The error was: ");
                    println!("{}\n", e);
                    println!("Project may be in inconsistent state. Please clean build after errors are resolved.");
                    return;
                }
            }
            let r = build();
            if let Err(e) = r {
                println!("\nAn error occurred whilst performing operation 'build'");
                println!("Check you are in the root of the sss project directory and the directory structure is valid.");
                println!("Also check the source markdown files are well formed. Proper syntax errors may be highlighted in the future.\n");
                println!("The error was: ");
                println!("{}\n", e);
                println!("Project may be in inconsistent state. Please clean build after errors are resolved.");
            }
        },
        ("clean", Some(_)) => { 
            let r = clean();
            if let Err(e) = r {
                println!("\nAn error occurred whilst performing operation 'clean'.");
                println!("Check you are in the root of sss project directory and that the directory structure is valid.\n");
                println!("The error was: ");
                println!("{}\n", e);
                println!("Project may be in inconsistent state. Please clean build after errors are resolved.");
            }
        },
        ("add", Some(sc_m)) => {
            if let (Some(t), Some(f)) = (sc_m.value_of("TITLE"), sc_m.value_of("FILE")) {
                let r = add(t, f);
                if let Err(e) = r {
                    println!("\nAn error occurred whilst performing operation 'add'");
                    println!("Check you are in the root of sss project directory and that the directory structure is valid.");
                    println!("Also check that the name and path to the post is valid.\n");
                    println!("The error was: ");
                    println!("{}\n", e);
                    println!("Project may be in inconsistent state. Please clean build after errors are resolved.") }
            } else {
                println!("Missing Arguments!");
            }
        }
        _ => println!("No subcommand specified. Please specify a subcommand")
    };

}
