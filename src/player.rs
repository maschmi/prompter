use std::process::Child;
use std::process::Command;
use std::collections::HashMap;


pub struct Player {
    playback_program: String,
    sound_process: HashMap<String, Child>
}

impl Player {
    pub fn new(program_path: &str) -> Self {
        Self {
            playback_program: program_path.to_string(),
            sound_process: HashMap::new()
        }
    }
}

impl Control for Player {

    fn play(&mut self, filepath: &str) -> Result<(), std::io::Error> {
        let child_process = Command::new(&self.playback_program)
            .arg(filepath.clone())
            .spawn()?;
        self.sound_process.insert(filepath.to_string(), child_process);
        Ok(())
    }

    fn stop(&mut self, filepath: &str) -> () {
        self.sound_process
            .remove(filepath)
            .unwrap().kill();
    }
}

pub trait Control {
    fn play(&mut self, filepath: &str) -> Result<(), std::io::Error>;
    fn stop(&mut self, filepath: &str) ->  ();
}

