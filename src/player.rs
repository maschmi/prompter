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

    fn play(&mut self, filepath: &str) -> Result<String, std::io::Error> {
        let child_process = Command::new(&self.playback_program)
            .arg(filepath.clone())
            .spawn()?;
        self.sound_process.insert(filepath.to_string(), child_process);
        Ok(filepath.to_string())
    }

    fn stop(&mut self, filepath: &str) -> Result<String, std::io::Error> {
        self.sound_process.remove(filepath)
            .unwrap().kill()?;
        Ok(filepath.to_string())
    }
}

pub trait Control {
    fn play(&mut self, filepath: &str) -> Result<String, std::io::Error>;
    fn stop(&mut self, filepath: &str) ->  Result<String, std::io::Error>;
}

