#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::epaint::text::{FontInsert, InsertFontFamily};
use ratatui::{
    backend::Backend,
    buffer::Cell,
    layout::Rect as RatatuiRect,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[derive(Debug, Default)]
pub struct DummyBackend {
    pub size: RatatuiRect,
    pub cursor: (u16, u16),
    pub cursor_shown: bool,
}

impl Backend for DummyBackend {
    // TODO i dont really need error here as it wont error ever
    type Error = std::io::Error;

    // NOTE all drawing is happening in egui from Terminal completed frame buffer
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

// TODO add theming
#[derive(Debug)]
struct RatatuiEguiApp {
    terminal: Terminal<DummyBackend>,
}

impl RatatuiEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);

        let backend = DummyBackend::default();
        let terminal = Terminal::new(backend).unwrap();
        Self { terminal }
    }
}

impl eframe::App for RatatuiEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());

            let (char_width, char_height) = ctx.fonts_mut(|f| {
                let glyph = f.glyph_width(&font_id, 'M');
                let row_h = f.row_height(&font_id);
                (glyph, row_h)
            });

            let char_size = egui::vec2(char_width, char_height);

            // TODO add padding so "terminal" is always centered
            let available_size = ui.available_size();
            let cols = (available_size.x / char_size.x).floor() as u16;
            let rows = (available_size.y / char_size.y).floor() as u16;

            // Update Ratatui's internal size
            self.terminal.backend_mut().size = RatatuiRect::new(0, 0, cols, rows);
            self.terminal.autoresize().unwrap();

            let painter = ui.painter();

            // draw to ratatui buffer
            let frame = self.terminal.draw(|f| {
                let block = Block::default()
                    .title(" Fixed Ratatui-in-Egui ")
                    .borders(Borders::ALL);

                f.render_widget(Paragraph::new("Fixed the font borrow error!").block(block), f.area());
            }).unwrap();

            // draw ratatui buffer to the screen
            let buffer = frame.buffer;
            for y in 0..buffer.area.height {
                for x in 0..buffer.area.width {
                    let pos = egui::Pos2::new(x as f32 * char_size.x, y as f32 * char_size.y);
                    let cell = buffer.get(x, y);

                    // draw bg
                    painter.rect_filled(
                        egui::Rect::from_min_size(pos, char_size),
                        0.0,
                        // NOTE bg defaults to black
                        map_color(cell.bg).unwrap_or(egui::Color32::BLACK),
                    );

                    // draw text
                    if cell.symbol() != " " {
                        painter.text(
                            pos,
                            egui::Align2::LEFT_TOP,
                            cell.symbol(),
                            font_id.clone(),
                            // NOTE fg defaults to white
                            map_color(cell.fg).unwrap_or(egui::Color32::WHITE),
                        );
                    }
                }
            }
        });
    }
}

fn map_color(color: ratatui::style::Color) -> Option<egui::Color32> {
    use ratatui::style::Color as RColor;
    match color {
        RColor::Reset => None,
        RColor::Black => Some(egui::Color32::BLACK),
        RColor::Red => Some(egui::Color32::from_rgb(204, 0, 0)),
        RColor::Green => Some(egui::Color32::from_rgb(78, 154, 6)),
        RColor::Yellow => Some(egui::Color32::from_rgb(196, 160, 0)),
        RColor::Blue => Some(egui::Color32::from_rgb(52, 101, 164)),
        RColor::Magenta => Some(egui::Color32::from_rgb(117, 80, 123)),
        RColor::Cyan => Some(egui::Color32::from_rgb(6, 152, 154)),
        RColor::Gray => Some(egui::Color32::from_rgb(211, 215, 207)),
        RColor::White => Some(egui::Color32::WHITE),
        RColor::Rgb(r, g, b) => Some(egui::Color32::from_rgb(r, g, b)),
        RColor::Indexed(i) => match i {
            // TODO
            _ => Some(egui::Color32::from_gray(i)),
        },

        // TODO some colors are missing
        _ => Some(egui::Color32::GRAY),
    }
}

// TODO embed few fonts into the library so the user doesnt have to do much
fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!(
            "../3270NerdFont-Regular.ttf"
        )),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Highest,
            },
        ],
    ));
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ratagui",
        native_options,
        Box::new(|cc| Ok(Box::new(RatatuiEguiApp::new(cc)))),
    )
}
