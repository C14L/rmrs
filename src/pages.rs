
extern crate dirs;

use std::env;
use std::io;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::{PathBuf};

#[get("/")]
pub fn home() -> &'static str {
    let html_ext = OsStr::new("html");
    let pagesdir: PathBuf = match env::var("PAGESDIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => PathBuf::from("/tmp"),
    };

    let htmlfiles: Vec<Result<DirEntry, io::Error>> = pagesdir
        .read_dir()
        .expect("Could not read pages dir.")
        .filter(|maybe_file| {
            match maybe_file {
                Ok(file) => {
                    match file.file_type() {
                        Ok(filetype) => {
                            match file.path().extension() {
                                Some(ext) => html_ext == ext && filetype.is_file(),
                                _ => false,
                            }
                        },
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        })
        .collect();

    println!("{:?}", htmlfiles);

    "Hello!"
}
