use ratatui::{
    backend::Backend,
    buffer::Cell,
    layout::Rect as RatatuiRect,
};

/// Ratagui backend, basically just holds the size and cursor info does nothing by itself all the
/// rendering is delegated to `Ratagui`
#[derive(Debug, Default)]
pub(crate) struct DummyBackend {
    pub size: RatatuiRect,
    pub cursor: (u16, u16),
    pub cursor_shown: bool,
}

impl Backend for DummyBackend {
    // TODO i dont really need error here as it wont error ever
    type Error = std::io::Error;

    // NOTE all drawing is happening in egui from buffer
    fn draw<'a, I: Iterator<Item = (u16, u16, &'a Cell)>>(&mut self, _content: I) -> std::io::Result<()> { Ok(()) }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        self.cursor_shown = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        self.cursor_shown = true;
        Ok(())
    }

    fn get_cursor_position(&mut self) -> std::io::Result<ratatui::layout::Position> {
        Ok(self.cursor.into())
    }

    fn set_cursor_position<P: Into<ratatui::prelude::Position>>(&mut self, position: P) -> std::io::Result<()> {
        let position: ratatui::prelude::Position = position.into();
        self.cursor = (position.x, position.y);
        Ok(())
    }

    fn clear(&mut self) -> std::io::Result<()> { Ok(()) }
    fn size(&self) -> std::io::Result<ratatui::layout::Size> { Ok(self.size.into()) }
    fn window_size(&mut self) -> std::io::Result<ratatui::prelude::backend::WindowSize> {
        Ok(ratatui::backend::WindowSize {
            columns_rows: self.size.into(),

            // TODO
            pixels: self.size.into(),
        })
    }
    fn clear_region(&mut self, _clear_type: ratatui::prelude::backend::ClearType) -> std::io::Result<()> { Ok(()) }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
