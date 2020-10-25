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
    pub previous_files: Option<TextAndAudioPair>,
    pub current_files: Option<TextAndAudioPair>,
    pub next_files: Option<TextAndAudioPair>,
    pub current_position: usize,
    pub total_entries: usize,
    file_pairs: Vec<TextAndAudioPair>
}

impl PrompterPlaylist {

    pub fn initialize(directory: &str) -> PrompterPlaylist {
        let files = FileLoader::load(directory);
        let size = files.len();
        PrompterPlaylist {
            previous_files: None,
            current_files: None,
            next_files: PrompterPlaylist::copy_file_entry(files.get(0)),
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

        let next_files = PrompterPlaylist::copy_file_entry(
            self.file_pairs.get(self.current_position + 1));
        let previous_files = PrompterPlaylist::copy_file_entry(self.current_files.as_ref());
        let curr_files = PrompterPlaylist::copy_file_entry(self.next_files.as_ref());

        PrompterPlaylist {
            previous_files: previous_files,
            current_files: curr_files,
            next_files,
            current_position: self.current_position + 1,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn move_back(&self) -> PrompterPlaylist {

        if self.current_position == 0 {
            return PrompterPlaylist {
                previous_files: None,
                current_files: None,
                next_files: PrompterPlaylist::copy_file_entry(self.file_pairs.get(0)),
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec()
            };
        }

        if self.current_position == 1 {
            return PrompterPlaylist {
                previous_files: None,
                current_files: PrompterPlaylist::copy_file_entry(self.file_pairs.get(0)),
                next_files: PrompterPlaylist::copy_file_entry(self.file_pairs.get(1)),
                current_position: 0,
                total_entries: self.total_entries,
                file_pairs: self.file_pairs.to_vec()
            };
        }

        let updated_position = self.current_position - 1;
        let next_files = PrompterPlaylist::copy_file_entry(
            self.file_pairs.get(self.current_position));

        let mut previous_files = None;
        if self.current_position > 2 {
            previous_files = PrompterPlaylist::copy_file_entry({
                self.file_pairs.get(self.current_position - 2)
            });
        }

        let curr_files = PrompterPlaylist::copy_file_entry(
            self.file_pairs.get(updated_position));

        PrompterPlaylist {
            previous_files,
            current_files: curr_files,
            next_files: next_files,
            current_position: updated_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }

    pub fn copy(&self) -> PrompterPlaylist {
        PrompterPlaylist {
            previous_files: PrompterPlaylist::copy_file_entry(self.previous_files.as_ref()),
            current_files: PrompterPlaylist::copy_file_entry(self.current_files.as_ref()),
            next_files: PrompterPlaylist::copy_file_entry(self.next_files.as_ref()),
            current_position: self.current_position,
            total_entries: self.total_entries,
            file_pairs: self.file_pairs.to_vec()
        }
    }
}