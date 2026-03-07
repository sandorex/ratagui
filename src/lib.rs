mod fonts;
mod backend;

use eframe::egui;
use egui::epaint::text::{FontInsert, InsertFontFamily};
use ratatui::{
    layout::Rect as RatatuiRect,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crate::backend::DummyBackend;

// TODO add theming
#[derive(Debug)]
struct Ratagui {
    terminal: Terminal<DummyBackend>,
}

impl Ratagui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        let font_name = "Departure Mono";
        fonts.font_data.insert(
            font_name.to_owned(),
            egui::FontData::from_static(fonts::DEPARTURE_MONO_NF).into()
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, font_name.to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, font_name.to_owned());

        let mut style = (*cc.egui_ctx.style()).clone();
        style.override_font_id = Some(egui::FontId::proportional(24.0));
        // style.spacing.scroll = egui::style::ScrollStyle::floating();
        cc.egui_ctx.set_style(style);

        // TODO adjust this with the theme
        cc.egui_ctx.set_pixels_per_point(2.);

        let backend = DummyBackend::default();
        let terminal = Terminal::new(backend).unwrap();
        Self { terminal }
    }
}

impl eframe::App for Ratagui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            // NOTE very important, egui weird margin by default
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());

            let char_size = ctx.fonts_mut(|f| egui::vec2(
                f.glyph_width(&font_id, 'M'),
                f.row_height(&font_id),
            ));

            let available_size = ui.available_size();
            let cols = (available_size.x / char_size.x).floor() as u16;
            let rows = (available_size.y / char_size.y).floor() as u16;

            let padding_x = (available_size.x % char_size.x) / 2.;
            let padding_y = (available_size.y % char_size.y) / 2.;

            // Update Ratatui's internal size
            self.terminal.backend_mut().size = RatatuiRect::new(0, 0, cols, rows);
            self.terminal.autoresize().unwrap();

            let painter = ui.painter();
            // TODO the background should be the same color as default bg color
            // painter.rect_filled(
            //     egui::Rect {
            //         min: egui::Pos2 { x: 0., y: 0. },
            //         max:egui::Pos2 { x: available_size.x, y: available_size.y }
            //     },
            //     0.0,
            //     egui::Color32::BLUE,
            // );

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
                    let pos = egui::Pos2::new(
                        x as f32 * char_size.x + padding_x,
                        y as f32 * char_size.y + padding_y,
                    );
                    let cell = buffer.cell(ratatui::prelude::Position::new(x, y)).unwrap();

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

            // ui.input(|i| &i.events);

            // TODO handle events and pass them to the widget
            // ui.input(|i| i.events)
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

/// Helper function to start ratagui easily
pub fn start(app_name: &str) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Ok(Box::new(Ratagui::new(cc)))),
    )
}

