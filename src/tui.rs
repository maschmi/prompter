/*use cursive::{Cursive, View, Printer};
use crate::fileloader::{PlaybackFiles, TextAndAudioPair};
use crate::player::{Player, Control};
use cursive::views::{TextView, SelectView, DummyView, FixedLayout, OnEventView};
use cursive::traits::Nameable;
use cursive::event::Event::{Key, Char};
use cursive::event::{EventResult, Event};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

pub struct PrompterTui {
    file_controller: PlaybackFiles,
    audio_controller: Player
}


    fn next(siv: &mut Cursive, file_controller: Rc<RefCell<&PlaybackFiles>>, audio_controller: Rc<RefCell<&Player>>) {
        PrompterTui::stop(file_controller, audio_controller);

        let mut fc = file_controller.deref().borrow_mut();
        let mut ac = audio_controller.deref().borrow_mut();
        if let Some(next_files) = fc.next() {
            if let Some(text) = next_files.text.clone() {
                PrompterTui::update_text_view(siv, text.as_str())
            } else {
                PrompterTui::update_text_view(siv, "Kein Text verfügbar.");
            }

            if let Some(audio_file) = next_files.audio.clone() {
                ac.play(audio_file.to_str().unwrap());
            }
        } else {
            PrompterTui::update_text_view(siv, "Nix mehr da.");
        }
    }

    fn stop(file_controller: Rc<RefCell<&PlaybackFiles>>, audio_controller: Rc<RefCell<&Player>>) {
        let mut fc = file_controller.deref().borrow_mut();
        let mut ac = audio_controller.deref().borrow_mut();

        if let Some(current_files) = fc.current() {
            if let Some(audio_file) = &current_files.audio {
                ac.stop(audio_file.to_str().unwrap());
            }
        }
    }

    fn update_text_view(siv: &mut Cursive, text_to_display: &str) {
        siv.call_on_name("text",
                              |view: &mut TextView | {
                                    view.set_content(text_to_display.to_string());
                              });
    }


    }

}
*/