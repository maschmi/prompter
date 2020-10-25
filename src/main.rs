use crate::player::{Control, Player};
use cursive::Cursive;
use cursive::views::{TextView, OnEventView, TextContentRef};
use std::borrow::{BorrowMut, Borrow};

use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::cell::RefCell;
use cursive::traits::Nameable;
use std::io;
use crate::playlist::PrompterPlaylist;
use std::process::exit;

mod fileloader;
mod player;
mod tui;
mod playlist;


enum PlaylistAction {
    NEXT,
    BACK,
    STOP
}

fn update(siv: &mut Cursive, current_files: PrompterPlaylist, audio_player: Weak<RefCell<Player>>, text: &str) {
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

fn stop_player(siv: &mut Cursive, file_list_ref: Rc<PrompterPlaylist>, audio_player_ref: Weak<RefCell<Player>>) {
    skip_impl(siv, file_list_ref, audio_player_ref, PlaylistAction::STOP);
}

fn next(siv: &mut Cursive, file_list_ref: Rc<PrompterPlaylist>, audio_player_ref: Weak<RefCell<Player>>)  {
    skip_impl(siv, file_list_ref, audio_player_ref, PlaylistAction::NEXT);
}

fn previous(siv: &mut Cursive, file_list_ref: Rc<PrompterPlaylist>, audio_player_ref: Weak<RefCell<Player>>)  {
    skip_impl(siv, file_list_ref, audio_player_ref, PlaylistAction::BACK);
}

fn skip_impl (siv: &mut Cursive, file_list_ref: Rc<PrompterPlaylist>, audio_player_ref: Weak<RefCell<Player>>, skip_direction: PlaylistAction) {
    if let Some(audio_player_upgrade) = audio_player_ref.upgrade() {
        let audio_player = audio_player_upgrade.deref();
        let file_list = file_list_ref.deref().copy();

        if let Some(file_to_stop) = file_list.get_current_files() {
            if let Some(fp) = file_to_stop.audio.as_ref() {
                audio_player.deref().borrow_mut().stop(fp.to_str().unwrap());
            }
        }

        match skip_direction {
            PlaylistAction::NEXT => {execute_skip(siv, audio_player_ref, audio_player, file_list.move_next(), "Nix mehr da");}
            PlaylistAction::BACK => {execute_skip(siv, audio_player_ref, audio_player, file_list.move_back(), "Am Anfang war nix.");}
            PlaylistAction::STOP => {execute_stop(siv, audio_player_ref, file_list)}
        }

    }
}

fn execute_stop(siv: &mut Cursive, audio_player_ref: Weak<RefCell<Player>>, file_list: PrompterPlaylist) {
    let text = siv.call_on_name("text", |v: &mut TextView|  { v.get_content() });
    siv.pop_layer();
    match text {
        Some(t) => update(siv.borrow_mut(), file_list, audio_player_ref, t.source()),
        _ => update(siv.borrow_mut(), file_list, audio_player_ref, "Kein Text verfügbar!")
    };
}

fn execute_skip(siv: &mut Cursive, audio_player_ref: Weak<RefCell<Player>>, audio_player: &RefCell<Player>, moved_file_list: PrompterPlaylist, placeholder_text: &str) {
    siv.pop_layer();
    let copied_List = moved_file_list.copy();
    let mut text = placeholder_text;
    if let Some(next_files) = copied_List.get_current_files() {
        text = "Kein Text verfügbar.";

        if let Some(txt) = next_files.text.as_ref() {
            text = txt;
        }

        if let Some(audio_file) = next_files.audio.as_ref() {
            audio_player.borrow_mut().play(audio_file.to_str().unwrap());
        }
    }
    update(siv.borrow_mut(), moved_file_list, audio_player_ref, text);
}




fn update_text_view(siv: &mut Cursive, text_to_display: &str) {
    siv.call_on_name("text",
                     |view: &mut TextView | {
                         view.set_content(text_to_display.to_string());
                     });
}


fn main()  -> io::Result<()> {

    let dir_to_search ="/home/martin/Documents";
    let initital_files = PrompterPlaylist::initialize(dir_to_search);

    let mut audio_player = player::Player::new("/usr/bin/mpg123", Some("-q"));
    let audio_player_ref = Rc::new(RefCell::new(audio_player));

    let mut siv = cursive::default();
    update(siv.borrow_mut(), initital_files, Rc::downgrade(&audio_player_ref), "Welcome to prompter.");
    siv.run();

    Ok(())
}
