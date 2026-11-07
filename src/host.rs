//! What Help tells the toolkit about itself: its words and the person's choices, read from the files Trindle keeps them in.

use crate::toolkit::host::{Act, Host};
use crate::toolkit::ui::Labels;
use trindle_toolkit::common::files::{home, read_env, trindle_conf};

struct Help;

fn setting(key: &str) -> String {
    read_env(&trindle_conf()).get(key).cloned().unwrap_or_default()
}

fn reader_on() -> bool {
    std::fs::read_to_string(home().join(".local/state/trindle/own-helpers")).is_ok_and(|t| t.lines().any(|l| l.trim() == "screen-reader"))
}

impl Host for Help {
    fn labels(&self) -> Labels {
        if reader_on() {
            Labels::Always
        } else if setting("labels_stay") == "on" {
            Labels::WhilePointing
        } else {
            Labels::Brief
        }
    }

    fn moving(&self) -> bool {
        std::env::var("TRINDLE_DRAWING").ok().as_deref() != Some("light")
    }

    fn asks_only_the_person(&self, _name: &str) -> bool {
        false
    }

    fn act(&self, _act: &str) -> Act {
        Act::Unknown
    }

    fn reader_switched_on(&self) -> bool {
        reader_on()
    }

    fn words(&self) -> &'static [(&'static str, &'static str)] {
        include!(concat!(env!("OUT_DIR"), "/english.rs"))
    }

    fn language(&self) -> Option<String> {
        Some(setting("language")).filter(|s| !s.trim().is_empty())
    }
}

/// Gives the toolkit Help's answers. Called before `main`, and so before every test.
pub extern "C" fn set() {
    crate::toolkit::host::set(&Help);
}
