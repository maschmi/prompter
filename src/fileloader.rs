use std::{fs, io};
use std::io::Error;
use std::fs::{DirEntry};
use std::ffi::OsString;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextAndAudioPair {
    pub audio: Option<OsString>,
    pub text: Option<String>
}

struct OrderedFiles {
    order_number: u32,
    path: OsString
}

pub struct FileLoader {
    dir_to_parse: String
}

impl FileLoader {

    fn load(directory: &str) -> Vec<TextAndAudioPair> {
        let mut instance = Self {
            dir_to_parse: directory.to_string()
        };
        instance.loadImpl()
    }

    fn loadImpl(&mut self) -> Vec<TextAndAudioPair> {
        let mp3_files = self.get_mp3_files().unwrap();
        let txt_files = self.get_text_files().unwrap();

        let text_map= FileLoader::parse_text_files(txt_files);
        let mut audio_map: HashMap<u32, OsString> = HashMap::new();
        mp3_files.iter()
            .for_each(|e| { audio_map.insert(e.order_number, e.path.to_os_string()); });


        let mut mp3_files_order = mp3_files.iter().map(|f| f.order_number).collect::<Vec<u32>>();

        let mut order_numbers = text_map.iter().map(|t| t.0.clone()).collect::<Vec<u32>>();
        order_numbers.append(&mut mp3_files_order);
        order_numbers.sort();
        order_numbers.dedup();

        order_numbers.iter().map(|on| TextAndAudioPair {
            text: text_map.get(on).cloned(),
            audio: audio_map.get(on).cloned()
        }).collect()
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
            .filter(|res| FileLoader::file_name_ends_on(res, ending))
            .filter_map(|entry| FileLoader::extract_path(entry))
            .map(|path| Ok(path))
            .collect::<Result<Vec<_>, io::Error>>()
    }

    fn extract_path(entry: Result<DirEntry, Error>) -> Option<OrderedFiles> {
        match entry {
            Ok(dir_entry) => Some(OrderedFiles {
                order_number: FileLoader::extract_order_number(dir_entry.file_name()),
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

pub struct PlaybackFiles {
    pub previous_files: Option<TextAndAudioPair>,
    pub current_files: Option<TextAndAudioPair>,
    pub next_files: Option<TextAndAudioPair>,
    pub current_position: usize,
    pub total_entries: usize,
    file_pairs: Vec<TextAndAudioPair>
}

impl PlaybackFiles {

    pub fn initialize(directory: &str) -> PlaybackFiles {
        let files = FileLoader::load(directory);
        let size = files.len();
        PlaybackFiles {
            previous_files: None,
            current_files: None,
            next_files: PlaybackFiles::copy_file_entry(files.get(0)),
            file_pairs: files,
            total_entries: size,
            current_position: 0,
        }
    }

    fn copy_file_entry(input: Option<&TextAndAudioPair>) -> Option<TextAndAudioPair>{
        if let Some(nxt) = input {
            return Some( TextAndAudioPair {
                audio: nxt.audio.as_ref().map(|a| OsString::from(a)),
                text: nxt.text.as_ref().map(|t| String::from(t))
                }
            );
        };
        Option::None
    }

    pub fn move_next(&self) -> PlaybackFiles {

        let next_files = PlaybackFiles::copy_file_entry(
            self.file_pairs.get(self.current_position + 1));
        let previous_files = PlaybackFiles::copy_file_entry(self.current_files.as_ref());
        let curr_files = PlaybackFiles::copy_file_entry(self.next_files.as_ref());

        PlaybackFiles {
            previous_files: previous_files,
            current_files: curr_files,
            next_files,
            current_position: self.current_position + 1,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn move_back(&self) -> PlaybackFiles {

        if self.current_position == 0 {
            return PlaybackFiles {
                previous_files: None,
                current_files: None,
                next_files: PlaybackFiles::copy_file_entry(self.file_pairs.get(0)),
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec()
            };
        }

        if self.current_position == 1 {
            return PlaybackFiles {
                previous_files: None,
                current_files: PlaybackFiles::copy_file_entry(self.file_pairs.get(0)),
                next_files: PlaybackFiles::copy_file_entry(self.file_pairs.get(1)),
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec()
            };
        }

        let updated_position = self.current_position - 1;
        let next_files = PlaybackFiles::copy_file_entry(
            self.file_pairs.get(self.current_position));

        let mut previous_files = None;
        if self.current_position > 2 {
            previous_files = PlaybackFiles::copy_file_entry({
                self.file_pairs.get(self.current_position - 2)
            });
        }

        let curr_files = PlaybackFiles::copy_file_entry(
            self.file_pairs.get(updated_position));

        PlaybackFiles {
            previous_files,
            current_files: curr_files,
            next_files: next_files,
            current_position: updated_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn copy(&self) -> PlaybackFiles {
        PlaybackFiles {
            previous_files: PlaybackFiles::copy_file_entry(self.previous_files.as_ref()),
            current_files: PlaybackFiles::copy_file_entry(self.current_files.as_ref()),
            next_files: PlaybackFiles::copy_file_entry(self.next_files.as_ref()),
            current_position: self.current_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }
}