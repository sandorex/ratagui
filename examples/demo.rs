#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use ratatui::widgets::{Block, Borders, Paragraph, Widget};

#[derive(Copy, Clone)]
struct DummyWidget;

impl ratagui::RataguiWidget for DummyWidget {
    fn handle_event(ratagui: &mut ratagui::Ratagui<Self, Self::State>, event: crossterm::event::Event)
        where <Self as ratatui::prelude::StatefulWidget>::State: Sized,
    {
        if event.is_key() {
            ratagui.close()
        }
    }
}

impl ratatui::widgets::StatefulWidget for DummyWidget {
    type State = ();

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _state: &mut Self::State) {
        let block = Block::default()
            .title(" Ratagui Demo ")
            .borders(Borders::ALL);

        Paragraph::new("This still looks like a terminal")
            .block(block)
            .render(area, buf);
    }
}

fn main() -> eframe::Result<()> {
    ratagui::start_simple(
        "Ratagui Demo",
        DummyWidget,
        ()
    )?;

    Ok(())
}
