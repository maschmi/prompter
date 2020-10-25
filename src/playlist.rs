use std::ffi::OsString;
use crate::fileloader::{SearchForPlaylistFiles, FileLoader};
use std::borrow::BorrowMut;

pub struct PlaylistInitConfig<T>  where T: SearchForPlaylistFiles {
    file_loader: T,
    directory_to_search: String,
}

#[derive(Clone)]
pub struct TextAndAudioPair {
    pub audio: Option<OsString>,
    pub text: Option<String>
}

pub struct PrompterPlaylist {
    pub current_position: usize,
    pub total_entries: usize,
    file_pairs: Vec<TextAndAudioPair>
}

impl PrompterPlaylist {

    pub fn initialize(directory: &str) -> PrompterPlaylist {
        let files = FileLoader::load(directory);

        let size = files.len();
        PrompterPlaylist {
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

    pub fn move_next(&self) -> PrompterPlaylist {

        let mut new_pos = self.current_position + 1;
        if new_pos > self.file_pairs.len() + 1 {
            new_pos = self.current_position;
        }



        PrompterPlaylist {
            current_position: new_pos,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn move_back(&self) -> PrompterPlaylist {

        let mut new_pos = 0;
        if self.current_position > 0 {
            new_pos = self.current_position - 1;
        }

        let previous_files = self.file_pairs.get(self.current_position);
        let current_files = self.file_pairs.get(new_pos);
        let next_files = self.file_pairs.get(new_pos + 1);

        PrompterPlaylist {
            current_position: new_pos,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn copy(&self) -> PrompterPlaylist {
        PrompterPlaylist {
            current_position: self.current_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn get_current_files(&self) -> Option<&TextAndAudioPair> {
        self.file_pairs.get(self.current_position)
    }

    pub fn get_previous_files(&self) -> Option<&TextAndAudioPair> {
        self.file_pairs.get(self.current_position - 1)
    }

    pub fn get_next_files(&self) -> Option<&TextAndAudioPair> {
        self.file_pairs.get(self.current_position + 1)
    }
}