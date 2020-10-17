use crate::player::{Control, Player};
use cursive::Cursive;
use cursive::views::{TextView, OnEventView, TextContentRef};
use std::borrow::{BorrowMut, Borrow};
use crate::fileloader::{ PlaybackFiles, TextAndAudioPair};
use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::cell::RefCell;
use cursive::traits::Nameable;
use std::io;

mod fileloader;
mod player;
mod tui;


fn update(siv: &mut Cursive, current_files:  PlaybackFiles, audio_player: Weak<RefCell<Player>>, text: &str) {
    let text_view = TextView::new(text).with_name("text");

    let new_files = Rc::new(current_files);

    let ref_files_stop = Rc::clone(&new_files);
    let ref_files_play = Rc::clone(&new_files);
    let ref_files_prev = Rc::clone(&new_files);

    let ref_player_stop = Weak::clone(&audio_player);
    let ref_player_play = Weak::clone(&audio_player);
    let ref_player_prev = Weak::clone(&audio_player);

    let event_view = OnEventView::new(text_view)
        .on_event('q', |s| s.quit())
        // .on_event('x', |s| s.toggle_debug_console())
        .on_event('a', move |s| previous(s, ref_files_prev.clone(), ref_player_prev.clone()))
        .on_event('s', move |s| stop_player(s, ref_files_stop.clone(), ref_player_stop.clone()))
        .on_event('d',  move |s| next(s, ref_files_play.clone(), ref_player_play.clone()));
    //.on_event('s', move |s| next(s, player.clone()));
    siv.add_layer(event_view);
}

fn stop_player(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Weak<RefCell<Player>>) {
    let strong_ref = audio_player_ref.upgrade().unwrap();
    let audio_player = strong_ref.deref();
    let file_list = file_list_ref.deref().copy();

    if let Some(file_to_stop) = file_list.current_files.borrow().as_ref() {
        if let Some(fp) = file_to_stop.audio.as_ref() {
            audio_player.deref().borrow_mut().stop(fp.to_str().unwrap());
        }
    }

    let text = siv.call_on_name("text", |v: &mut TextView|  { v.get_content() });
    siv.pop_layer();
    match text {
        Some(t) => update(siv.borrow_mut(), file_list, audio_player_ref, t.source()),
        _ => update(siv.borrow_mut(), file_list, audio_player_ref, "Kein Text verfügbar!")
    };
}

fn next(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Weak<RefCell<Player>>)  {
    if let Some(audio_player_upgrade) = audio_player_ref.upgrade() {
        let strong_ref = audio_player_ref.upgrade().unwrap();
        let audio_player = strong_ref.deref();
        let file_list = file_list_ref.deref().copy();

        if let Some(file_to_stop) = file_list.current_files.borrow().as_ref() {
            if let Some(fp) = file_to_stop.audio.as_ref() {
                audio_player.deref().borrow_mut().stop(fp.to_str().unwrap());
            }
        }
        siv.pop_layer();

        if let Some(next_files) = file_list.next_files.as_ref() {
            let moved_list = file_list.move_next();
            let mut text = "Kein Text verfügbar.";

            if let Some(txt) = next_files.text.as_ref() {
                text = txt;
            }

            if let Some(audio_file) = next_files.audio.as_ref() {
                audio_player.borrow_mut().play(audio_file.to_str().unwrap());
                //.expect(panic!("Uh oh"));
                update(siv.borrow_mut(), moved_list, audio_player_ref, text);
            } else {
                update(siv.borrow_mut(), moved_list, audio_player_ref, text);
            }
        } else {
            update(siv.borrow_mut(), file_list, audio_player_ref, "Nix mehr da.")
        }
    }
}
fn previous(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Weak<RefCell<Player>>)  {
    if let Some(audio_player_upgrade) = audio_player_ref.upgrade() {
        let strong_ref = audio_player_ref.upgrade().unwrap();
        let audio_player = strong_ref.deref();
        let file_list = file_list_ref.deref().copy();

        let file_to_stop = file_list.current_files.borrow().as_ref().unwrap();

        if let Some(file_to_stop) = file_list.current_files.borrow().as_ref() {
            if let Some(fp) = file_to_stop.audio.as_ref() {
                audio_player.deref().borrow_mut().stop(fp.to_str().unwrap());
            }
        }

        siv.pop_layer();

        if let Some(prev_files) = file_list.previous_files.as_ref() {
            let moved_list = file_list.move_back();
            let mut text = "Kein Text verfügbar.";

            if let Some(txt) = prev_files.text.as_ref() {
                text = txt;
            }

            if let Some(audio_file) = prev_files.audio.as_ref() {
                audio_player.borrow_mut().play(audio_file.to_str().unwrap().borrow());
                //.expect(panic!("Uh oh"));
                update(siv.borrow_mut(), moved_list, audio_player_ref, text);
            } else {
                update(siv.borrow_mut(), moved_list, audio_player_ref, text);
            }
        } else {
            update(siv.borrow_mut(), file_list, audio_player_ref, "Am Anfang war nix.")
        }
    }
}


fn update_text_view(siv: &mut Cursive, text_to_display: &str) {
    siv.call_on_name("text",
                     |view: &mut TextView | {
                         view.set_content(text_to_display.to_string());
                     });
}


fn main()  -> io::Result<()> {

    let dir_to_search ="/home/martin/Documents";
    let initital_files = PlaybackFiles::initialize(dir_to_search);

    let mut audio_player = player::Player::new("/usr/bin/mpg123", Some("-q"));
    let audio_player_ref = Rc::new(RefCell::new(audio_player));

    let mut siv = cursive::default();
    update(siv.borrow_mut(), initital_files, Rc::downgrade(&audio_player_ref), "Welcome to prompter.");
    siv.run();

    Ok(())
}
