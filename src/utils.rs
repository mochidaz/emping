use std::{env, fs};
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};

use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub fn config() -> Yaml {
    let string = read_file(&PathBuf::from("config.yaml")).unwrap();
    let docs = YamlLoader::load_from_str(string.as_str()).expect("Failed");
    docs[0].clone()
}

pub fn read_lines(file: impl AsRef<Path>)
                  -> Vec<String> {
    let f = File::open(file)
        .expect("File not found");
    let buffer = io::BufReader::new(f);
    buffer.lines()
        .map(|x| x.expect("Could not parse line"))
        .collect()
}

pub fn read_file(file: &PathBuf)
                 -> io::Result<String> {
    let mut f = File::open(file)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}


pub fn is_project(dir: Option<&str>) -> bool {
    let current_dir = env::current_dir().unwrap().to_owned();
    let config_file = Path::new("config.yaml");
    let d = match dir {
        Some(p) => Path::new(p),
        None => Path::new(&current_dir),
    };
    if Path::new(&format!("{}/{}", d.display(), config_file.display())).exists() {
        return true;
    }
    false
}

pub fn get_files(path: Option<&str>) -> Vec<PathBuf> {

    let mut post_list = match path {
        Some(p) => {
            fs::read_dir(p).unwrap()
                .filter(|res| res.as_ref().map(|e| e
                    .path()
                    .extension()
                    .unwrap() == OsStr::new("md"))
                    .unwrap())
                .map(|res| res.map(|e| e.
                    path()))
                .collect::<Result<Vec<PathBuf>, io::Error>>()
                .unwrap()
        }
        None => {
            fs::read_dir("./_posts").unwrap()
                .filter(|res| res.as_ref().map(|e| e
                    .path()
                    .extension()
                    .unwrap() == OsStr::new("md"))
                    .unwrap())
                .map(|res| res.map(|e| e
                    .path()))
                .collect::<Result<Vec<PathBuf>, io::Error>>()
                .unwrap()
        }
    };

    post_list.sort();
    post_list
}

pub fn write_to_file(html: &String, filename: &String, dir: PathBuf) -> io::Result<()> {
    fs::create_dir_all(&dir)?;
    fs::File::create(format!("{}/{}.html", dir.display(),filename))?
        .write_all(html.as_bytes())?;
    Ok(())
}