use std::process::Child;
use std::process::Command;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::Deref;


pub struct Player {
    playback_program: String,
    std_args: String,
    sound_process: Option<Rc<RefCell<Child>>>,
    copied_process: Option<Weak<RefCell<Child>>>,
    playing_file: Option<String>
}

impl Player {

    pub fn new(program_path: &str, default_args: Option<&str>) -> Self {
        Self {
            playback_program: program_path.to_string(),
            std_args: match default_args {
                Some(arg) => arg.to_string(),
                None => "".to_string()
            },
            sound_process: None,
            copied_process: None,
            playing_file: None
        }
    }

    pub fn copy(&self) -> Player {
        let mut copied_process = None;
        if let Some(parent_proc) = &self.sound_process.as_ref() {
            copied_process = Some(Rc::downgrade(parent_proc));
        }

        Player {
            playback_program: String::from(&self.playback_program),
            sound_process: None,
            copied_process: copied_process,
            std_args: String::from(&self.std_args),
            playing_file: self.playing_file.as_ref().map(|s| String::from(s))
        }
    }
}

impl Control for Player {

    fn play(&self, filepath: &str) -> Result<Player, std::io::Error> {
        let child_process = Command::new(&self.playback_program)
            .args(&[&self.std_args, filepath.clone()])
            .spawn()?;
        Ok(Player{
            playback_program: String::from(&self.playback_program),
            std_args: String::from(&self.std_args),
            playing_file: Some(String::from(filepath)),
            sound_process: Some(Rc::new(RefCell::new(child_process))),
            copied_process: None
        })
    }

    fn stop(&self) -> Result<Player, std::io::Error> {
        if let Some(owner) = self.sound_process.as_ref() {
            owner.deref().borrow_mut().kill();
        } else if let Some(copy) = self.copied_process.as_ref() {
            copy.upgrade().unwrap().deref().borrow_mut().kill();
        }

        Ok(Player {
            playback_program: String::from(&self.playback_program),
            std_args: String::from(&self.std_args),
            playing_file: None,
            sound_process: None,
            copied_process: None
        })
    }

    fn empty(&self) -> Result<Player, std::io::Error> {
        Ok(Player {
            playback_program: String::from(&self.playback_program),
            std_args: String::from(&self.std_args),
            playing_file: None,
            sound_process: None,
            copied_process: None
        })
    }

}

pub trait Control {
    fn play(&self, filepath: &str) -> Result<Player, std::io::Error>;
    fn stop(&self) ->  Result<Player, std::io::Error>;
    fn empty(&self) -> Result<Player, std::io::Error>;
}

