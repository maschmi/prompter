use crate::player::Control;
use std::{io, time};

mod fileloader;
mod player;

fn main()  -> io::Result<()> {
    let mut audio_playback = player::Player::new("/usr/bin/mpg123", Some("-Tq"));
    let dir_to_search ="/home/martin/Documents";

    let mut loader = fileloader::PlaybackFiles::new(dir_to_search);
    loader.load();

    while let Some(current_files) = loader.next() {

        if let Some(audio_file) =  current_files.audio.as_ref() {
            match audio_playback.play(audio_file.to_str().unwrap()) {
                Ok(song) => println!("Playing {}", song),
                Err(e) => panic!(e.to_string())
            };
        }

        if let Some(text) = current_files.text.as_ref() {
            println!("Text: {}", text);
        } else {
            println!("No text available");
        }


        let pause = time::Duration::from_millis(5000);

        std::thread::sleep(pause);
        if let Some(audio_file) =  current_files.audio.as_ref() {
            match audio_playback.stop(audio_file.to_str().unwrap()) {
                Ok(f) => println!("{} was stopped", f),
                Err(e) => panic!(e.to_string())
            };
        }
    }
    Ok(())
}
