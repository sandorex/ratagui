mod fonts;
mod theme;
mod backend;
mod conversion;

use eframe::egui;
use ratatui_core::{terminal::Terminal, layout::Rect as RatatuiRect, widgets::StatefulWidget};
use crate::{backend::DummyBackend, theme::Theme};

/// Trait that allows handling window input
pub trait RataguiWidget: StatefulWidget + Copy {
    /// Handle egui events translated into crossterm events
    fn handle_event(_state: &mut Self::State, _event: crossterm::event::Event) {}

    /// Handle raw egui events, if true then event wont be passed on to `handle_event`
    fn handle_raw_event(_state: &mut Self::State, _event: egui::Event) -> bool { false }
}

#[derive(Debug)]
pub struct Ratagui<W: RataguiWidget<State = S>, S> {
    terminal: Terminal<DummyBackend>,
    theme: Theme,
    widget: W,
    state: S,
}

impl<W: RataguiWidget<State = S>, S> Ratagui<W, S> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        widget: W,
        state: S,
        theme: Theme,
        font: Option<&'static [u8]>,
        font_size: f32,
    ) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        // just make the requested font the default
        let font_name = "default";
        fonts.font_data.insert(
            font_name.to_owned(),
            egui::FontData::from_static(font.unwrap_or(fonts::DEPARTURE_MONO_NF)).into()
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

        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_pixels_per_point(font_size);

        let backend = DummyBackend::default();
        let terminal = Terminal::new(backend).unwrap();

        Self { terminal, theme, widget, state, }
    }
}

impl<W: RataguiWidget<State = S>, S> eframe::App for Ratagui<W, S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            // NOTE very important, egui adds weird margin by default
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());

            // TODO possibly cache this? as it does not change ever
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

            // draw to ratatui buffer
            let frame = self.terminal.draw(|f| {
                f.render_stateful_widget(self.widget, f.area(), &mut self.state);
            }).unwrap();

            // draw background
            painter.rect_filled(
                egui::Rect {
                    min: egui::Pos2 { x: 0., y: 0. },
                    max:egui::Pos2 { x: available_size.x, y: available_size.y }
                },
                0.0,
                self.theme.background,
            );

            // TODO draw cursor if not hidden

            // draw ratatui buffer to the screen
            let buffer = frame.buffer;
            for y in 0..buffer.area.height {
                for x in 0..buffer.area.width {
                    let pos = egui::Pos2::new(
                        x as f32 * char_size.x + padding_x,
                        y as f32 * char_size.y + padding_y,
                    );
                    let cell = buffer.cell(ratatui_core::layout::Position::new(x, y)).unwrap();

                    // draw bg
                    painter.rect_filled(
                        egui::Rect::from_min_size(pos, char_size),
                        0.0,
                        self.theme.map_color(cell.bg).unwrap_or(self.theme.background),
                    );

                    // draw text
                    if cell.symbol() != " " {
                        painter.text(
                            pos,
                            egui::Align2::LEFT_TOP,
                            cell.symbol(),
                            font_id.clone(),
                            self.theme.map_color(cell.fg).unwrap_or(self.theme.foreground),
                        );
                    }
                }
            }

            // redirect all events to the widget
            ui.input(|i| {
                if !i.events.is_empty() {
                    for event in &i.events {
                        if !W::handle_raw_event(&mut self.state, event.clone()) {
                            if let Some(crossterm_event) = conversion::convert_event(event) {
                                W::handle_event(&mut self.state, crossterm_event);
                            }
                        }
                    }
                }
            });
        });
    }
}

/// Start ratagui, it just creates new `Ratagui` instance and starts with `eframe::run_native`
pub fn start<W: RataguiWidget<State = S>, S>(
    app_name: &str,
    widget: W,
    state: S,
    theme: Theme,
    font: Option<&'static [u8]>,
    font_size: Option<f32>,
) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Ok(Box::new(Ratagui::new(
            cc,
            widget,
            state,
            theme,
            font,
            font_size.unwrap_or(1.0),
        )))),
    )
}

/// Same as `start` but with the least arguments
pub fn start_simple<W: RataguiWidget<State = S>, S>(app_name: &str, widget: W, state: S) -> eframe::Result<()> {
    start(app_name, widget, state, Theme::default(), None, None)
}
