use crate::player::Control;
use std::borrow::Borrow;
use std::error::Error;
use std::{fs, io, time};
use std::any::Any;

mod fileloader;
mod player;

fn main()  -> io::Result<()> {
    let mut audio_playback = player::Player::new("/usr/bin/mpg123");
    let dir_to_search ="/home/martin/Documents";
    let mut entries = fs::read_dir(dir_to_search)?
        .map(|res| res.map(|e| e.path()))
        .filter(|res| {
            match res {
                Ok(path) => path.file_name().unwrap().to_str().unwrap().ends_with("mp3"),
                Err(e) => false
            }
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    println!("Parsing {}", dir_to_search);
    for x in entries {
        println!("Evaluating {:?}", x);
        println!("Starting {}", x.to_str().unwrap());
        let song = match audio_playback.play(x.clone().to_str().unwrap()) {
            Ok(_) => x.to_str().unwrap(),
            Err(e) => panic!(e.to_string())
        };
        println!("{}", song);
        let pause = time::Duration::from_millis(10000);
        std::thread::sleep(pause);
        audio_playback.stop(x.clone().to_str().unwrap());
    }
    Ok(())
}
