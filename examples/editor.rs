use edtui::{EditorState, EditorEventHandler, EditorTheme, EditorView};
use ratatui::widgets::Widget;

#[derive(Copy, Clone)]
struct EditorWidget;

// TODO symbols just arent passed through?
impl ratagui::RataguiWidget for EditorWidget {
    fn handle_event(state: &mut Self::State, event: crossterm::event::Event) {
        let mut event_handler = EditorEventHandler::default();
        match event {
            crossterm::event::Event::Key(x) => if x.is_press() || x.is_repeat() {
                event_handler.on_key_event(x, state)
            },
            _ => {}
        }
    }
}

impl ratatui::widgets::StatefulWidget for EditorWidget {
    type State = EditorState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        EditorView::new(state)
            .theme(EditorTheme::default())
            .wrap(true)
            .syntax_highlighter(None)
            .tab_width(2)
            .render(area, buf);
    }
}

fn main() -> eframe::Result<()> {
    ratagui::start_simple(
        "Ratagui Editor Demo",
        EditorWidget,
        EditorState::default(),
    )?;

    Ok(())
}
