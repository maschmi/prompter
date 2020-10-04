use crate::player::Control;
use std::{fs, io, time};

mod fileloader;
mod player;

fn main()  -> io::Result<()> {
    let mut audio_playback = player::Player::new("/usr/bin/mpg123", Some("-Tq"));
    let dir_to_search ="/home/martin/Documents";
    let entries = fileloader::get_mp3_files(dir_to_search).expect("Couĺd not open directory!");

    for x in entries {
        match audio_playback.play(x.to_str().unwrap()) {
            Ok(song) => println!("Playing {}", song),
            Err(e) => panic!(e.to_string())
        };

        let pause = time::Duration::from_millis(10000);
        std::thread::sleep(pause);
        match audio_playback.stop(x.to_str().unwrap()) {
            Ok(f) => println!("{} was stopped", f),
            Err(e) => panic!(e.to_string())
        };
    }
    Ok(())
}
