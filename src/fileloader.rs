use std::{fs, io};
use std::io::Error;
use std::fs::{DirEntry};
use std::ffi::OsString;
use std::collections::HashMap;


pub struct FilePairs {
    pub audio: Option<OsString>,
    pub text: Option<String>
}

pub struct PlaybackFiles {
    counter: usize,
    file_pairs: Vec<FilePairs>,
    dir_to_parse: String
}

impl PlaybackFiles {
    pub fn new(dir: &str) -> Self {
        Self {
            counter: 0,
            dir_to_parse: dir.to_string(),
            file_pairs: Vec::new()
        }
    }

    pub fn load(&mut self) -> () {
        let mp3_files = self.get_mp3_files().unwrap();
        let txt_files= self.get_text_files().unwrap();
        let text = PlaybackFiles::parse_text_files(txt_files);
        let text_values = text.iter().map(|f| f.1).collect::<Vec<&String>>();
        self.file_pairs = mp3_files.iter().zip(text_values.iter())
            .map(|z| FilePairs {
                audio: Some(z.0.to_os_string()),
                text: Some(z.1.to_string())
            })
            .collect::<Vec<FilePairs>>();
    }

    pub fn next(&mut self) -> &FilePairs {
        let pair = &self.file_pairs[self.counter];
        self.counter += 1;
        pair
    }

    fn get_mp3_files(&mut self) -> Result<Vec<OsString>, io::Error> {
        let mut files = self.read_dir_and_filter("mp3")?;
        files.sort();
        Ok(files)
    }

    fn get_text_files(&mut self) -> Result<Vec<OsString>, io::Error> {
        let mut files = self.read_dir_and_filter("txt")?;
        files.sort();
        Ok(files)
    }

    fn read_dir_and_filter(&mut self, ending: &str) -> Result<Vec<OsString>, io::Error> {
        fs::read_dir(self.dir_to_parse.clone())?
            .filter(|res| PlaybackFiles::file_name_ends_on(res, ending))
            .filter_map(|entry| PlaybackFiles::extract_path(entry))
            .map(|path| Ok(path))
            .collect::<Result<Vec<_>, io::Error>>()
    }

    fn extract_path(entry: Result<DirEntry, Error>) -> Option<OsString> {
        match entry {
            Ok(dir_entry) => Some(dir_entry.path().into_os_string()),
            Err(e) => {
                println!("Error getting path: {}", e.to_string());
                None
            }
        }
    }

    fn file_name_ends_on(res: &Result<DirEntry, Error>, ending: &str) -> bool {
        res.as_ref().unwrap().file_name().to_str().unwrap().ends_with(ending)
    }

    fn parse_text_files(files: Vec<OsString>) -> HashMap<OsString, String>{
        let mut map = HashMap::new();
        files
            .iter()
            .for_each(
                |f| { map.insert(f.to_os_string(), fs::read_to_string(f).unwrap()); }
            );
        map
    }
}

