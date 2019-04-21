
extern crate dirs;

use rocket::response::content::Html;
use rocket::State;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::{PathBuf};

pub fn fetch_pages_filenames() -> Vec<DirEntry> {
    // only get .html files
    let html_ext = OsStr::new("html");
    // pages dir is set with a environment variable
    let pagesdir: PathBuf = match env::var("PAGESDIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => PathBuf::from("/tmp"),
    };
    // get only files with the correct extension
    pagesdir.read_dir().expect("Could not read pages dir.")
        .filter_map(|file| file.ok())
        .filter(|file| {
            match file.file_type() {
                Ok(filetype) => filetype.is_file(),
                Err(_) => false,
            }
        })
        .filter(|file| {
            match file.path().extension() {
                Some(ext) => html_ext == ext,
                _ => false,
            }
        })
        .collect()
}

#[get("/")]
pub fn home(filenames: State<'_, Vec<DirEntry>>) -> Html<String> {
    let pagename = OsStr::new("index.html");
    let pagepath = filenames.inner().into_iter().find(|x| x.file_name() == pagename);
    let pagehtml = match pagepath {
        Some(p) => fs::read_to_string(p.path()),
        None => panic!("Oh noes!"),
    };
    match pagehtml {
        Ok(s) => Html(format!("{}", s)),
        Err(_) => Html(String::from("")),
    }
}
