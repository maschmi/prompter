use crate::player::{Control, Player};
use std::{io, time};
use cursive::Cursive;
use cursive::views::{TextView, OnEventView};
use std::borrow::{BorrowMut, Borrow};
use crate::fileloader::FileLoader;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use cursive::traits::Nameable;
use std::ffi::OsString;

mod fileloader;
mod player;
mod tui;


fn configure(siv: &mut Cursive, loader:  Rc<RefCell<FileLoader>>, player: Rc<RefCell<Player>>) {
    let text_view = TextView::new("Welcome to prompter").with_name("text");
    let lc = loader.clone();
    let event_view = OnEventView::new(text_view)
        .on_event('q', |s| s.quit())
        // .on_event('x', |s| s.toggle_debug_console())
        .on_event('s', |s| next(s, lc, player.clone()))
        .on_event('d', move |s| next(s, loader.clone(), player.clone()));
    siv.add_layer(event_view);
}

fn next(siv: &mut Cursive, loader_ref: Rc<RefCell<FileLoader>>, player_ref: Rc<RefCell<Player>>) {
    let mut loader = loader_ref.deref().borrow_mut();
    let mut audio_playback = player_ref.deref().borrow_mut();

    if let Some(current_files) = loader.current() {
        if let Some(audio_file) = &current_files.audio {
            audio_playback.stop(audio_file.to_str().unwrap());
        }
    }

    if let Some(next_files) = loader.next() {
        if let Some(text) = next_files.text.as_ref() {
            update_text_view(siv.borrow_mut(), text.as_str())
        } else {
            update_text_view(siv.borrow_mut(), "Kein Text verfügbar.")
        }

        if let Some(audio_file) = next_files.audio.as_ref() {
            audio_playback.play(audio_file.to_str().unwrap());
        }
    } else {
        update_text_view(siv.borrow_mut(), "Nix mehr da!");
    }
}

fn update_text_view(siv: &mut Cursive, text_to_display: &str) {
    siv.call_on_name("text",
                     |view: &mut TextView | {
                         view.set_content(text_to_display.to_string());
                     });
}

fn stop(loader_ref: Rc<RefCell<FileLoader>>, player_ref: Rc<RefCell<Player>>) {
    let mut fc = loader_ref.deref().borrow_mut();
    let mut ac = player_ref.deref().borrow_mut();

    if let Some(current_files) = fc.current() {
        if let Some(audio_file) = &current_files.audio {
            ac.stop(audio_file.to_str().unwrap());
        }
    }
}

fn main()  -> io::Result<()> {

    let dir_to_search ="/home/martin/Documents";

    let mut loader = fileloader::FileLoader::new(dir_to_search);
    loader.load();
    let mut audio_playback = player::Player::new("/usr/bin/mpg123", Some("-q"));

    let loader_rc = Rc::new(RefCell::new(loader));
    let player_rc = Rc::new(RefCell::new(audio_playback));

    let mut siv = cursive::default();
    configure(siv.borrow_mut(), loader_rc, player_rc);
    siv.run();

    Ok(())
}
