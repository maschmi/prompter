use crate::player::Control;
use std::{io, time};

mod fileloader;
mod player;

fn main()  -> io::Result<()> {
    let mut audio_playback = player::Player::new("/usr/bin/mpg123", Some("-Tq"));
    let dir_to_search ="/home/martin/Documents";
    let mut loader = fileloader::PlaybackFiles::new(dir_to_search);
    loader.load();
    let current_files = loader.next();
    let current_audio_file = current_files.audio.as_ref().unwrap().to_str().unwrap();
    let current_text = current_files.text.as_ref().unwrap();
    match audio_playback.play(current_audio_file) {
        Ok(song) => println!("Playing {}", song),
        Err(e) => panic!(e.to_string())
    };
    println!("Text: {}", current_text);
    let pause = time::Duration::from_millis(10000);
    std::thread::sleep(pause);
    match audio_playback.stop(current_audio_file) {
        Ok(f) => println!("{} was stopped", f),
        Err(e) => panic!(e.to_string())
    };
    Ok(())
}
