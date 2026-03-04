use crossterm::event::{Event, KeyEvent, KeyEventState, KeyModifiers, MouseEvent};
use macroquad::prelude::*;

pub trait MacroquadEventHandler {
    fn handle_event(&mut self, event: crossterm::event::Event);
}

pub struct EventHandlerProxy<'a>(pub &'a mut dyn MacroquadEventHandler);

impl macroquad::miniquad::EventHandler for EventHandlerProxy<'_> {
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn key_down_event(&mut self, keycode: KeyCode, keymods: miniquad::KeyMods, repeat: bool) {
        // TODO idk if all keys map
        let Some(code) = keycode_to_keycode(keycode) else {
            return;
        };

        self.0.handle_event(Event::Key(KeyEvent {
            code,
            modifiers: keymods_to_key_modifiers(keymods),
            kind: if repeat { crossterm::event::KeyEventKind::Repeat } else { crossterm::event::KeyEventKind::Press },
            state: KeyEventState::NONE,
        }))
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: miniquad::KeyMods) {
        let Some(code) = keycode_to_keycode(keycode) else {
            return;
        };

        self.0.handle_event(Event::Key(KeyEvent {
            code,
            modifiers: keymods_to_key_modifiers(keymods),
            kind: crossterm::event::KeyEventKind::Release,
            state: KeyEventState::NONE,
        }))
    }

    // TODO how do i do this? i dont have access to the backend here, make it all one big struct?
    // fn mouse_motion_event(&mut self, x: f32, y: f32) {
    //     self.0.handle_event(Event::Mouse(MouseEvent {
    //         kind: crossterm::event::MouseEventKind::Moved,
    //         column:,
    //         row: ,
    //         modifiers
    //     }))
    // }
}

fn keycode_to_keycode(keycode: miniquad::KeyCode) -> Option<crossterm::event::KeyCode> {
    use crossterm::event::KeyCode as CKeyCode;

    // TODO there are a lot of them
    match keycode {
        KeyCode::Enter => Some(CKeyCode::Enter),
        _ => None,
    }
}

fn keymods_to_key_modifiers(keymods: miniquad::KeyMods) -> KeyModifiers {
    let mut mods = KeyModifiers::empty();

    if keymods.alt {
        mods.set(KeyModifiers::ALT, true);
    }

    if keymods.ctrl {
        mods.set(KeyModifiers::CONTROL, true);
    }

    if keymods.shift {
        mods.set(KeyModifiers::SHIFT, true);
    }

    if keymods.logo {
        // TODO META or SUPER?
        mods.set(KeyModifiers::SUPER, true);
    }

    mods
}
