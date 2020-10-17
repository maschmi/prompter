use crate::player::{Control, Player};
use cursive::Cursive;
use cursive::views::{TextView, OnEventView, TextContentRef};
use std::borrow::{BorrowMut, Borrow};
use crate::fileloader::{ PlaybackFiles, TextAndAudioPair};
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use cursive::traits::Nameable;
use std::io;

mod fileloader;
mod player;
mod tui;


fn update(siv: &mut Cursive, current_files:  PlaybackFiles, audio_player: Player, text: &str) {
    let text_view = TextView::new(text).with_name("text");

    let new_files = Rc::new(current_files);
    let new_player = Rc::new(audio_player);

    let ref_files_stop = Rc::clone(&new_files);
    let ref_files_play = Rc::clone(&new_files);
    let ref_files_prev = Rc::clone(&new_files);

    let ref_player_stop = Rc::clone(&new_player);
    let ref_player_play = Rc::clone(&new_player);
    let ref_player_prev = Rc::clone(&new_player);

    let event_view = OnEventView::new(text_view)
        .on_event('q', |s| s.quit())
        // .on_event('x', |s| s.toggle_debug_console())
        .on_event('a', move |s| previous(s, ref_files_prev.clone(), ref_player_prev.clone()))
        .on_event('s', move |s| stop_player(s, ref_files_stop.clone(), ref_player_stop.clone()))
        .on_event('d',  move |s| next(s, ref_files_play.clone(), ref_player_play.clone()));
    //.on_event('s', move |s| next(s, player.clone()));
    siv.add_layer(event_view);
}

fn stop_player(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Rc<Player>) {
    let audio_player = audio_player_ref.deref();
    let file_list = file_list_ref.deref().copy();

    let stopped_player = audio_player.stop().unwrap();
    let text = siv.call_on_name("text", |v: &mut TextView|  { v.get_content() });
    siv.pop_layer();
    match text {
        Some(t) => update(siv.borrow_mut(), file_list, stopped_player, t.source()),
        _ => update(siv.borrow_mut(), file_list, stopped_player, "Kein Text verfügbar!")
    };
}

fn next(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Rc<Player>)  {
    let audio_player = audio_player_ref.deref();
    let file_list = file_list_ref.deref().copy();

    let stopped_player = stop(file_list.current_files.borrow(), audio_player.copy());

    siv.pop_layer();

    if let Some(next_files) = file_list.next_files.as_ref() {
        let moved_list = file_list.move_next();
        let mut text = "Kein Text verfügbar.";

        if let Some(txt) = next_files.text.as_ref() {
            text = txt;
        }

        if let Some(audio_file) = next_files.audio.as_ref() {
            let player = audio_player.play(audio_file.to_str().unwrap()).unwrap();
                //.expect(panic!("Uh oh"));
            update(siv.borrow_mut(), moved_list, player, text);
        } else {
            update(siv.borrow_mut(), moved_list, stopped_player, text);
        }
    } else {
        update(siv.borrow_mut(), file_list, stopped_player, "Nix mehr da.")
    }
}
fn previous(siv: &mut Cursive, file_list_ref: Rc<PlaybackFiles>, audio_player_ref: Rc<Player>)  {
    let audio_player = audio_player_ref.deref();
    let file_list = file_list_ref.deref().copy();

    let stopped_player = stop(file_list.current_files.borrow(), audio_player.copy());

    siv.pop_layer();

    if let Some(next_files) = file_list.previous_files.as_ref() {
        let moved_list = file_list.move_back();
        let mut text = "Kein Text verfügbar.";

        if let Some(txt) = next_files.text.as_ref() {
            text = txt;
        }

        if let Some(audio_file) = next_files.audio.as_ref() {
            let player = audio_player.play(audio_file.to_str().unwrap()).unwrap();
            //.expect(panic!("Uh oh"));
            update(siv.borrow_mut(), moved_list, player, text);
        } else {
            update(siv.borrow_mut(), moved_list, stopped_player, text);
        }
    } else {
        update(siv.borrow_mut(), file_list, stopped_player, "Nix mehr da.")
    }
}


fn update_text_view(siv: &mut Cursive, text_to_display: &str) {
    siv.call_on_name("text",
                     |view: &mut TextView | {
                         view.set_content(text_to_display.to_string());
                     });
}

fn stop(file_to_stop: &Option<TextAndAudioPair>, audio_player: Player) -> Player {
    if let Some(current_files) = file_to_stop {
        if let Some(_) = &current_files.audio {
            return audio_player.stop().unwrap();
        }
    }
    audio_player.empty().unwrap()
}

fn main()  -> io::Result<()> {

    let dir_to_search ="/home/martin/Documents";
    let initital_files = PlaybackFiles::initialize(dir_to_search);

    let audio_player = player::Player::new("/usr/bin/mpg123", Some("-q"));

    let mut siv = cursive::default();
    update(siv.borrow_mut(), initital_files, audio_player, "Welcome to prompter.");
    siv.run();

    Ok(())
}
