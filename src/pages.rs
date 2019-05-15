extern crate dirs;

use rocket::response::content::Html;
use rocket::State;
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;

pub fn fetch_pages_filenames() -> Vec<fs::DirEntry> {
    // only get .html files
    let html_ext = OsStr::new("html");
    // pages dir is set with a environment variable
    let pagesdir: PathBuf = match env::var("PAGESDIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => PathBuf::from("/tmp"),
    };
    // get only files with the correct extension
    pagesdir
        .read_dir()
        .expect("Could not read pages dir.")
        .filter_map(|file| file.ok())
        .filter(|file| match file.file_type() {
            Ok(filetype) => filetype.is_file(),
            Err(_) => false,
        })
        .filter(|file| match file.path().extension() {
            Some(ext) => html_ext == ext,
            _ => false,
        })
        .collect()
}

pub fn preload_static_pages(pages: &mut HashMap<OsString, String>) {
    for f in fetch_pages_filenames() {
        let k = f.file_name();
        let v = fs::read_to_string(f.path());

        match v {
            Ok(html) => &pages.insert(k, html),
            Err(_) => panic!("Aaaaaahhhh!"),
        };
    }
}

#[get("/")]
pub fn home(html_pages: State<'_, HashMap<OsString, String>>) -> Html<String> {
    let pagename = OsStr::new("index.html");

    match html_pages.get(pagename) {
        Some(s) => Html(s.clone()),
        None => Html(String::from("Not found!")),
    }
}

#[get("/settings.html")]
pub fn settings(html_pages: State<'_, HashMap<OsString, String>>) -> Html<String> {
    let pagename = OsStr::new("settings.html");

    match html_pages.get(pagename) {
        Some(s) => Html(s.clone()),
        None => Html(String::from("Not found!")),
    }
}
