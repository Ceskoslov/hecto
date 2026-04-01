use crate::prelude::*;

use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]
pub enum System {
    Save,
    Quit,
    Resize(Size),
    Dismiss,
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('s') => Ok(System::Save),
                Char('q') => Ok(System::Quit),
                Char('f') => Ok(System::Search),
                _ => Err(format!("Unknown control key: {:?}", code)),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(System::Dismiss)
        } else {
            Err(format!("Unknown key: {:?}", code))
        }
    }
}
