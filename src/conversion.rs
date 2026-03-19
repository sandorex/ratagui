//! Contains code to convert between egui and ratatui/crossterm types
#![allow(unused)]

mod gui {
    pub use egui::{Modifiers, Key, Event};
}

mod tui {
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventState, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
}

/// Tries to convert `egui::Event` into `crossterm::event::Event`
pub fn convert_event(
    event: &gui::Event,
    global_modifiers: &gui::Modifiers,
    size: egui::Vec2,
    term_size: (u16, u16),
) -> Option<tui::Event> {
    match event {
        gui::Event::Key { key, physical_key, pressed, repeat, modifiers } => {
            match physical_key.unwrap_or(*key) {
                // NOTE space is handled in text event!
                gui::Key::Escape |
                gui::Key::Tab |
                gui::Key::Delete |
                gui::Key::Insert |
                gui::Key::Home |
                gui::Key::PageUp |
                gui::Key::PageDown |
                gui::Key::Enter |
                gui::Key::Backspace |
                gui::Key::ArrowUp |
                gui::Key::ArrowDown |
                gui::Key::ArrowLeft |
                gui::Key::ArrowRight |
                gui::Key::F1 |
                gui::Key::F2 |
                gui::Key::F3 |
                gui::Key::F4 |
                gui::Key::F5 |
                gui::Key::F6 |
                gui::Key::F7 |
                gui::Key::F8 |
                gui::Key::F9 |
                gui::Key::F10 |
                gui::Key::F11 |
                gui::Key::F12 => convert_key_event(key, *pressed, *repeat, modifiers).map(|x| tui::Event::Key(x)),

                _ => None,
            }
        },

        gui::Event::Text(x) => {
            Some(tui::Event::Key(tui::KeyEvent {
                code: tui::KeyCode::Char(x.chars().next().unwrap()),
                modifiers: convert_modifiers(global_modifiers),
                kind: tui::KeyEventKind::Press,
                state: tui::KeyEventState::NONE,
            }))
        },

        gui::Event::Paste(x) => Some(tui::Event::Paste(x.clone())),

        // TODO use size and term_size to calculate where mouse events happened
        // // TODO map to the cells??
        // gui::Event::PointerMoved(pos) => Some(tui::Event::Mouse(tui::MouseEvent {
        //     kind: tui::MouseEventKind::Moved,
        //     row: 0,
        //     column: 0,
        //     modifiers: tui::KeyModifiers::NONE,
        // })),

        gui::Event::WindowFocused(focused) => Some(if *focused {
            tui::Event::FocusGained
        } else {
            tui::Event::FocusLost
        }),
        _ => None
    }
}

pub fn convert_key_event(key: &gui::Key, pressed: bool, repeat: bool, modifiers: &gui::Modifiers) -> Option<tui::KeyEvent> {
    let key_kind = if repeat {
        tui::KeyEventKind::Repeat
    } else if pressed {
        tui::KeyEventKind::Press
    } else {
        tui::KeyEventKind::Release
    };

    Some(tui::KeyEvent::new_with_kind_and_state(
        convert_key(key)?,
        convert_modifiers(modifiers),
        key_kind,
        tui::KeyEventState::NONE
    ))
}

pub fn convert_focus(focused: bool) -> tui::Event {
    if focused {
        tui::Event::FocusGained
    } else {
        tui::Event::FocusLost
    }
}

pub fn convert_key(key: &gui::Key) -> Option<tui::KeyCode> {
    use gui::Key;
    use tui::KeyCode;

    match key {
        Key::ArrowDown => Some(KeyCode::Down),
        Key::ArrowLeft => Some(KeyCode::Left),
        Key::ArrowRight => Some(KeyCode::Right),
        Key::ArrowUp => Some(KeyCode::Up),
        Key::Escape => Some(KeyCode::Esc),
        Key::Tab => Some(KeyCode::Tab),
        Key::Backspace => Some(KeyCode::Backspace),
        Key::Enter => Some(KeyCode::Enter),
        Key::Space => Some(KeyCode::Char(' ')),
        Key::Insert => Some(KeyCode::Insert),
        Key::Delete => Some(KeyCode::Delete),
        Key::Home => Some(KeyCode::Home),
        Key::End => Some(KeyCode::End),
        Key::PageUp => Some(KeyCode::PageUp),
        Key::PageDown => Some(KeyCode::PageDown),

        Key::F1 => Some(KeyCode::F(1)),
        Key::F2 => Some(KeyCode::F(2)),
        Key::F3 => Some(KeyCode::F(3)),
        Key::F4 => Some(KeyCode::F(4)),
        Key::F5 => Some(KeyCode::F(5)),
        Key::F6 => Some(KeyCode::F(6)),
        Key::F7 => Some(KeyCode::F(7)),
        Key::F8 => Some(KeyCode::F(8)),
        Key::F9 => Some(KeyCode::F(9)),
        Key::F10 => Some(KeyCode::F(10)),
        Key::F11 => Some(KeyCode::F(11)),
        Key::F12 => Some(KeyCode::F(12)),

        _ => None,
    }
}

// TODO cannot see meta/super
pub fn convert_modifiers(modifiers: &gui::Modifiers) -> tui::KeyModifiers {
    let mut result = tui::KeyModifiers::empty();

    result.set(tui::KeyModifiers::CONTROL, modifiers.ctrl || modifiers.command);
    result.set(tui::KeyModifiers::ALT, modifiers.alt);
    result.set(tui::KeyModifiers::SHIFT, modifiers.shift);

    result
}

