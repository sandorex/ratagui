//! Example using edtui editor as the sole widget
//!
//! You are editing the actual source code of the application

use edtui::{EditorState, EditorEventHandler, EditorTheme, EditorView};
use ratatui::widgets::Widget;

#[derive(Clone)]
struct EditorWidgetState {
    editor_state: EditorState,
    event_handler: EditorEventHandler,
}

impl EditorWidgetState {
    pub fn new() -> Self {
        Self {
            // for example just include this file itself
            editor_state: EditorState::new(edtui::Lines::from(include_str!("editor.rs"))),
            event_handler: EditorEventHandler::vim_mode(),
        }
    }
}

#[derive(Copy, Clone)]
struct EditorWidget;

impl ratagui::RataguiWidget for EditorWidget {
    fn handle_event(ctx: &mut ratagui::Context, state: &mut Self::State, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(x) => match x.code {
                // resize font with F1 and F2
                crossterm::event::KeyCode::F(f) => {
                    if x.is_release() || x.is_repeat() {
                        if f == 1 {
                            ctx.incr_font_size(-0.01);
                        } else if f == 2 {
                            ctx.incr_font_size(0.01);
                        }
                    }

                    // NOTE edtui crashes if it receives function keys
                    return;
                },
                _ => {},
            },
            _ => {},
        }

        // duplicates the key presses
        if event.is_key_release() {
            return;
        }

        state.event_handler.on_event(event, &mut state.editor_state);
    }
}

impl ratatui::widgets::StatefulWidget for EditorWidget {
    type State = EditorWidgetState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let syntax_highlighter = edtui::SyntaxHighlighter::new("monokai", "rs")
            .expect("Could not load syntax highlighter");

        EditorView::new(&mut state.editor_state)
            .theme(EditorTheme::default())
            .syntax_highlighter(Some(syntax_highlighter))
            .line_numbers(edtui::LineNumbers::Absolute)
            .wrap(true)
            .tab_width(2)
            .render(area, buf);
    }
}

fn main() -> eframe::Result<()> {
    ratagui::start_simple(
        "Ratagui Edtui Demo",
        EditorWidget,
        EditorWidgetState::new(),
    )?;

    Ok(())
}
