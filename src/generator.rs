use std::path::PathBuf;

use crate::parser::{get_frontmatter, md_to_html, parse_and_find_content, parse_filename, parse_time};
use crate::post::{Layout, Post};
use crate::utils::{config, get_files, read_file, write_to_file};

pub fn vec_of_posts(paths: &Vec<PathBuf>) -> Vec<Post> {
    paths.iter().map(|p| {
        let filename = parse_filename(&p).unwrap();
        let frontmatters = get_frontmatter(&p);
        let markdown_content = read_file(&p).unwrap();
        let time = parse_time(&frontmatters["date"].as_str().unwrap().to_string());
        let content = parse_and_find_content(&markdown_content.as_str()).unwrap().1.to_string();
        Post::new(
            match &frontmatters["title"].as_str() {
                Some(t) => {
                    t.replace(&[',', ' '][..], "-")
                        .to_string()
                },
                None => filename.to_string(),
            },
            content,
            match &frontmatters["layout"].as_str() {
                Some(l) => {
                    let lower = l.to_lowercase().clone();
                    match lower.as_str() {
                        "post" => Layout::Post,
                        "main" => Layout::Main,
                        "page" => Layout::Page,
                        _ => Layout::Default
                    }
                }
                None => Layout::Default,
            },
            match &frontmatters["permalink"].as_str() {
                Some(p) => p.to_string(),
                None => format!("/{}", &filename)
            },
            time.to_string()
        )
    }).collect::<Vec<Post>>()
}

pub(crate) fn generate_html(x: &Vec<Post>) {
    let conf = config();
    x.iter().for_each(|p| {
        write_to_file(&md_to_html(&p.content),
                      &p.title,
                      PathBuf::from(
            match conf["production_dir"].as_str() {
                Some(d) => d,
                None => "./web"
        }));
    });
}