use std::process::Child;
use std::process::Command;
use std::collections::HashMap;
use std::borrow::BorrowMut;


pub struct Player {
    playback_program: String,
    std_args: String,
    sound_process: HashMap<String, Child>
}

impl Player {
    pub fn new(program_path: &str, default_args: Option<&str>) -> Self {
        Self {
            playback_program: program_path.to_string(),
            std_args: match default_args {
                Some(arg) => arg.to_string(),
                None => "".to_string()
            },
            sound_process: HashMap::new()
        }
    }
}

impl Control for Player {

    fn play(&mut self, filepath: &str) -> Result<String, std::io::Error> {
        let child_process = Command::new(&self.playback_program)
            .args(&[&self.std_args, filepath.clone()])
            .spawn()?;
        self.sound_process.insert(filepath.to_string(), child_process);
        Ok(filepath.to_string())
    }

    fn stop(&mut self, filepath: &str) -> Result<String, std::io::Error> {
        if let Some(mut child) = self.sound_process.remove(filepath) {
            child.kill();
        }

        Ok(filepath.to_string())
    }
}

pub trait Control {
    fn play(&mut self, filepath: &str) -> Result<String, std::io::Error>;
    fn stop(&mut self, filepath: &str) ->  Result<String, std::io::Error>;
}

