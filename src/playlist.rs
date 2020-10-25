use std::ffi::OsString;
use crate::fileloader::{SearchForPlaylistFiles, FileLoader};

#[derive(Clone)]
pub struct TextAndAudioPair {
    pub audio: Option<OsString>,
    pub text: Option<String>
}

pub struct PrompterPlaylist {
    pub current_position: usize,
    pub total_entries: usize,
    file_pairs: Vec<TextAndAudioPair>,
    first_next: bool,
    last_back: bool
}

impl PrompterPlaylist {

    pub fn initialize(directory: &str) -> PrompterPlaylist {
        let files = FileLoader::load(directory);

        let size = files.len();
        PrompterPlaylist {
            file_pairs: files,
            total_entries: size,
            current_position: 0,
            first_next: true,
            last_back: true
        }
    }

    pub fn move_next(&self) -> PrompterPlaylist {

        if self.first_next {
            return PrompterPlaylist {
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec(),
                first_next: false,
                last_back: false
            }
        }

        let mut new_pos = self.current_position + 1;
        if new_pos > self.total_entries {
            new_pos = self.current_position;
        }

        PrompterPlaylist {
            current_position: new_pos,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec(),
            first_next: false,
            last_back: false
        }

    }

    pub fn move_back(&self) -> PrompterPlaylist {

        let mut new_pos: usize;
        if self.current_position > 0 {
            new_pos = self.current_position - 1;
        } else {
            return PrompterPlaylist {
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec(),
                first_next: true,
                last_back: true
            }
        }

        if self.current_position > self.total_entries {
            new_pos = self.total_entries - 1;
        }
        PrompterPlaylist {
            current_position: new_pos,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec(),
            first_next: self.first_next,
            last_back: false
        }
    }

    pub fn copy(&self) -> PrompterPlaylist {
        PrompterPlaylist {
            current_position: self.current_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec(),
            first_next: self.first_next,
            last_back: self.last_back
        }
    }

    pub fn get_current_files(&self) -> Option<&TextAndAudioPair> {
        if self.last_back {
            return None
        }
        self.file_pairs.get(self.current_position)
    }

    pub fn get_previous_files(&self) -> Option<&TextAndAudioPair> {
        if self.last_back {
            return None
        }
        self.file_pairs.get(self.current_position - 1)
    }

    pub fn get_next_files(&self) -> Option<&TextAndAudioPair> {
        self.file_pairs.get(self.current_position + 1)
    }
}