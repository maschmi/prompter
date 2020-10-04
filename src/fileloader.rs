use std::{fs, io};
use std::net::Shutdown::Read;
use std::io::Error;
use std::fs::{ReadDir, DirEntry};
use std::ffi::OsString;

pub fn get_mp3_files(directory: &str) -> Result<Vec<OsString>, io::Error> {
    let mut files = read_dir_and_filter(directory, "mp3")?;
    files.sort();
    Ok(files)
}

fn read_dir_and_filter(directory: &str, ending: &str) -> Result<Vec<OsString>, io::Error> {
    fs::read_dir(directory)?
        .filter(|res| file_name_ends_on(res, ending))
        .filter_map(|entry| extract_path(entry))
        .map(|path| Ok(path))
        .collect::<Result<Vec<_>, io::Error>>()
}

fn extract_path(entry: Result<DirEntry, Error>) -> Option<OsString> {
    match entry {
        Ok(dir_entry) => Some(dir_entry.path().into_os_string()),
        Err(e) => {
            println!("Error getting path: {}", e.to_string());
            None }
    }
}

fn file_name_ends_on(res: &Result<DirEntry, Error>, ending: &str) -> bool {
    res.as_ref().unwrap().file_name().to_str().unwrap().ends_with(ending)
}