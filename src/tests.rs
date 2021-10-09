use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::generator::generate_html;
use crate::generator::vec_of_posts;
use crate::parser::{get_frontmatter, md_to_html, parse, parse_and_find_content, parse_filename, parse_time, remove};
use crate::post::{Layout, Post};
use crate::utils::{config, get_files, is_project, read_file, read_lines, write_to_file};

#[test]
fn generate() {
    let conf = config();
    let posts_path = conf["posts_dir"].as_str();
    let markdown_files = get_files(posts_path);
    let vec = vec_of_posts(&markdown_files);
    generate_html(&vec);
}

#[test]
fn display_test() {
    let conf = config();
    let posts_path = conf["posts_dir"].as_str();
    let markdown_files = get_files(posts_path);
    println!("{:?}", vec_of_posts(  &markdown_files))
}

