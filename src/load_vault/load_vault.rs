use rayon::prelude::*;
use std::fs;
// use std::path::Path;
// use std::thread;
use walkdir::WalkDir;

use crate::note_structure::note_structure::parse_note;


pub fn load_vault_fn() {
    let entries: Vec<std::path::PathBuf> = WalkDir::new("./Bible Study Kit (v1)")
        .into_iter()
        .filter_map(|entry_res: Result<walkdir::DirEntry, walkdir::Error>| {
            let entry: walkdir::DirEntry = entry_res.ok()?;
            let ft = entry.file_type();
            if ft.is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext.eq_ignore_ascii_case("md") {
                        return Some(entry.path().to_path_buf());
                    }
                }
                None
            } else {
                None
            }
        })
        .collect();

    // println!("{:#?}", entries);

    entries.par_iter().for_each(|path| {
        // println!("Thread: {:?}", thread::current().id());
        if path.is_file() {
            match fs::read_to_string(path) {
                Ok(_) => {
                    parse_note(path);
                }
                Err(e) => {
                    eprintln!("Skipping {}: {}", path.display(), e);
                }
            }
        }
    });
}
