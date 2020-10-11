use std::{fs, io};
use std::io::Error;
use std::fs::{DirEntry};
use std::ffi::OsString;
use std::collections::HashMap;

pub struct TextAndAudioPair {
    pub audio: Option<OsString>,
    pub text: Option<String>
}

struct OrderedFiles {
    order_number: u32,
    path: OsString
}

pub struct PlaybackFiles {
    counter: usize,
    audio_text: Vec<TextAndAudioPair>,
    dir_to_parse: String
}

impl PlaybackFiles {
    pub fn new(dir: &str) -> Self {
        Self {
            counter: 0,
            dir_to_parse: dir.to_string(),
            audio_text: Vec::new()
        }
    }

    pub fn load(&mut self) -> () {
        let mp3_files = self.get_mp3_files().unwrap();
        let txt_files = self.get_text_files().unwrap();

        let text_map= PlaybackFiles::parse_text_files(txt_files);
        let mut audio_map: HashMap<u32, OsString> = HashMap::new();
        mp3_files.iter()
            .for_each(|e| { audio_map.insert(e.order_number, e.path.to_os_string()); });


        let mut mp3_files_order = mp3_files.iter().map(|f| f.order_number).collect::<Vec<u32>>();

        let mut order_numbers = text_map.iter().map(|t| t.0.clone()).collect::<Vec<u32>>();
        order_numbers.append(&mut mp3_files_order);
        order_numbers.sort();
        order_numbers.dedup();

        self.audio_text = order_numbers.iter().map(|on| TextAndAudioPair {
            text: text_map.get(on).cloned(),
            audio: audio_map.get(on).cloned()
        }).collect();
    }

    pub fn next(&mut self) -> Option<&TextAndAudioPair> {
        if self.counter >= self.audio_text.len() {
            return None;
        }
        let pair = &self.audio_text[self.counter];
        self.counter += 1;
        Some(pair)
    }

    fn get_mp3_files(&mut self) -> Result<Vec<OrderedFiles>, io::Error> {
        let files = self.read_dir_and_filter("mp3")?;
        Ok(files)
    }

    fn get_text_files(&mut self) -> Result<Vec<OrderedFiles>, io::Error> {
        let files = self.read_dir_and_filter("txt")?;
        Ok(files)
    }

    fn read_dir_and_filter(&mut self, ending: &str) -> Result<Vec<OrderedFiles>, io::Error> {
        fs::read_dir(self.dir_to_parse.clone())?
            .filter(|res| PlaybackFiles::file_name_ends_on(res, ending))
            .filter_map(|entry| PlaybackFiles::extract_path(entry))
            .map(|path| Ok(path))
            .collect::<Result<Vec<_>, io::Error>>()
    }

    fn extract_path(entry: Result<DirEntry, Error>) -> Option<OrderedFiles> {
        match entry {
            Ok(dir_entry) => Some(OrderedFiles {
                order_number: PlaybackFiles::extract_order_number(dir_entry.file_name()),
                path: dir_entry.path().into_os_string() }
            ),
            Err(e) => {
                println!("Error getting path: {}", e.to_string());
                None
            }
        }
    }

    fn extract_order_number(filename: OsString) -> u32 {
        let name = filename.to_str().unwrap();
        let parts = name.split(" - ").collect::<Vec<_>>();
        match parts[0].parse(){
            Ok(o) => o,
            Err(_) => panic!("Unsupported filename detected {}. Files need to begin with 000 - ", name)
        }
    }

    fn file_name_ends_on(res: &Result<DirEntry, Error>, ending: &str) -> bool {
        res.as_ref().unwrap().file_name().to_str().unwrap().ends_with(ending)
    }

    fn parse_text_files(files: Vec<OrderedFiles>) -> HashMap<u32, String>{
        let mut map: HashMap<u32, String> = HashMap::new();
        files
            .iter()
            .for_each(
                |f| { map.insert(f.order_number, fs::read_to_string(f.path.clone()).unwrap()); }
            );
        map
    }
}

