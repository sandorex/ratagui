//! Contains code to convert between egui and ratatui/crossterm types
#![allow(unused)]

mod gui {
    pub use egui::{Modifiers, Key, Event};
}

mod tui {
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventState, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
}

/// Tries to convert `egui::Event` into `crossterm::event::Event`
pub fn convert_event(event: &gui::Event) -> Option<tui::Event> {
    match event {
        gui::Event::Key { key, pressed, repeat, modifiers, .. } => {
            convert_key_event(key, *pressed, *repeat, modifiers).map(|x| tui::Event::Key(x))
        },
        // gui::Event::Text(x) // TODO how do i map this?
        gui::Event::Paste(x) => Some(tui::Event::Paste(x.clone())),

        // // TODO map to the cells??
        // gui::Event::PointerMoved(pos) => Some(tui::Event::Mouse(tui::MouseEvent {
        //     kind: tui::MouseEventKind::Moved,
        //     row: 0,
        //     column: 0,
        //     modifiers: tui::KeyModifiers::NONE,
        // })),

        gui::Event::WindowFocused(focused) => Some(if *focused { tui::Event::FocusGained } else { tui::Event::FocusLost }),
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

        Key::Num0 => Some(KeyCode::Char('0')),
        Key::Num1 => Some(KeyCode::Char('1')),
        Key::Num2 => Some(KeyCode::Char('2')),
        Key::Num3 => Some(KeyCode::Char('3')),
        Key::Num4 => Some(KeyCode::Char('4')),
        Key::Num5 => Some(KeyCode::Char('5')),
        Key::Num6 => Some(KeyCode::Char('6')),
        Key::Num7 => Some(KeyCode::Char('7')),
        Key::Num8 => Some(KeyCode::Char('8')),
        Key::Num9 => Some(KeyCode::Char('9')),

        Key::A => Some(KeyCode::Char('a')),
        Key::B => Some(KeyCode::Char('b')),
        Key::C => Some(KeyCode::Char('c')),
        Key::D => Some(KeyCode::Char('d')),
        Key::E => Some(KeyCode::Char('e')),
        Key::F => Some(KeyCode::Char('f')),
        Key::G => Some(KeyCode::Char('g')),
        Key::H => Some(KeyCode::Char('h')),
        Key::I => Some(KeyCode::Char('i')),
        Key::J => Some(KeyCode::Char('j')),
        Key::K => Some(KeyCode::Char('k')),
        Key::L => Some(KeyCode::Char('l')),
        Key::M => Some(KeyCode::Char('m')),
        Key::N => Some(KeyCode::Char('n')),
        Key::O => Some(KeyCode::Char('o')),
        Key::P => Some(KeyCode::Char('p')),
        Key::Q => Some(KeyCode::Char('q')),
        Key::R => Some(KeyCode::Char('r')),
        Key::S => Some(KeyCode::Char('s')),
        Key::T => Some(KeyCode::Char('t')),
        Key::U => Some(KeyCode::Char('u')),
        Key::V => Some(KeyCode::Char('v')),
        Key::W => Some(KeyCode::Char('w')),
        Key::X => Some(KeyCode::Char('x')),
        Key::Y => Some(KeyCode::Char('y')),
        Key::Z => Some(KeyCode::Char('z')),

        Key::Comma => Some(KeyCode::Char(',')),
        Key::Period => Some(KeyCode::Char('.')),
        Key::Colon => Some(KeyCode::Char(':')),
        Key::Semicolon => Some(KeyCode::Char(';')),
        Key::Questionmark => Some(KeyCode::Char('?')),
        Key::Plus => Some(KeyCode::Char('+')),
        Key::Minus => Some(KeyCode::Char('-')),
        Key::Slash => Some(KeyCode::Char('/')),
        Key::Backslash => Some(KeyCode::Char('\\')),
        Key::Equals => Some(KeyCode::Char('=')),
        Key::OpenBracket => Some(KeyCode::Char('[')),
        Key::CloseBracket => Some(KeyCode::Char(']')),
        Key::Backtick => Some(KeyCode::Char('`')),
        Key::Quote => Some(KeyCode::Char('\'')),

        _ => None,
    }
}

// TODO cannot see meta/super
pub fn convert_modifiers(modifiers: &gui::Modifiers) -> tui::KeyModifiers {
    let mut result = tui::KeyModifiers::empty();

    if modifiers.ctrl || modifiers.command {
        result.set(tui::KeyModifiers::CONTROL, true);
    }

    if modifiers.alt {
        result.set(tui::KeyModifiers::ALT, true);
    }

    if modifiers.shift {
        result.set(tui::KeyModifiers::SHIFT, true);
    }

    result
}

