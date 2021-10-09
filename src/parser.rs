use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use extract_frontmatter::Extractor;
use pulldown_cmark::{html, Options, Parser};
use yaml_rust::{ScanError, Yaml, YamlLoader};

use crate::post::Post;
use crate::utils::{read_file, read_lines};

pub fn parse_time(time: &String) -> NaiveDateTime {
    let parse = match NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%d %H:%M")
        .or(NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%d")) {
        Ok(p) => p,
        Err(e) => {
            let local_date = Local::now().naive_local().date();
            let local_hours = NaiveTime::parse_from_str(Local::now()
                                                            .time()
                                                            .format("%H:%M")
                                                            .to_string()
                                                            .as_str(), "%H:%M").unwrap();
            NaiveDateTime::new(local_date, local_hours)
        }
    };
    parse
}

// https://github.com/azdle/rust-frontmatter
fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("---\n");
            if fm_end.is_none() {
                return None
            };

            let fm_end = fm_end.unwrap();
            Some((4,fm_end+4, fm_end+2*4))
        },
        false => None
    }
}

// https://github.com/azdle/rust-frontmatter
pub fn parse_and_find_content(text: &str) -> Result<(Option<Yaml>, &str), ScanError> {
    match find_yaml_block(text) {
        Some((fm_start, fm_end, content_start)) => {
            let yaml_str = &text[fm_start..fm_end];
            let mut documents = YamlLoader::load_from_str(yaml_str)?;

            let rest_of_text = &text[content_start..];

            Ok((documents.pop(), rest_of_text))
        },
        None => Ok((None, text))
    }
}

// https://github.com/azdle/rust-frontmatter
pub fn parse(text: &str) -> Result<Option<Yaml>, ScanError> {
    let (matter, _) = parse_and_find_content(text)?;
    Ok(matter)
}

pub fn get_frontmatter(file: &PathBuf) -> Yaml {
    let mut read = read_file(file).unwrap();
    let mut extractor = Extractor::new(read.as_str());
    extractor.select_by_terminator("---");
    let output = extractor.extract();
    let load = YamlLoader::load_from_str(output.as_str()).unwrap();
    let doc = &load[0];
    doc.clone()
}

pub fn remove(file: &PathBuf) -> String {
    let mut read = read_file(file).unwrap();
    let mut extractor = Extractor::new(read.as_str());
    extractor.select_by_terminator("---");
    let output: &str = extractor.remove().trim();

    output.to_string()
}

pub fn md_to_html(content: &String) -> String {

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(content.as_str(), options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn parse_filename(filename: &PathBuf) -> Option<String> {
    match filename.file_stem() {
        Some(f) => Some(f.to_str().unwrap().to_string().clone()),
        None => None
    }
}