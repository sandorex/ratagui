mod fonts;
mod theme;
mod backend;
mod conversion;

use eframe::egui;
use ratatui_core::{terminal::Terminal, layout::Rect as RatatuiRect, widgets::StatefulWidget};
use crate::{backend::DummyBackend, theme::Theme};

/// Trait that allows handling window input
#[allow(unused)]
pub trait RataguiWidget: StatefulWidget + Copy {
    /// Handle egui events translated into crossterm events
    fn handle_event(ratagui: &mut Ratagui<Self, Self::State>, event: crossterm::event::Event)
        where <Self as StatefulWidget>::State: Sized,
    {}

    /// Handle raw egui events, if true then event wont be passed on to `handle_event`
    fn handle_raw_event(ratagui: &mut Ratagui<Self, Self::State>, event: egui::Event) -> bool
        where <Self as StatefulWidget>::State: Sized,
    { false }
}

#[derive(Debug)]
pub struct Ratagui<W: RataguiWidget<State = S>, S> {
    terminal: Terminal<DummyBackend>,
    theme: Theme,
    widget: W,
    pub state: S,
    font_changed: bool,
    font_size: f32,
    cell_size: egui::Vec2,
    quit_requested: bool,
}

impl<W: RataguiWidget<State = S>, S> Ratagui<W, S> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        widget: W,
        state: S,
        theme: Theme,
        font_size: f32,
        font: Option<&'static [u8]>,
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

        let backend = DummyBackend::default();
        let terminal = Terminal::new(backend).unwrap();

        Self {
            terminal,
            theme,
            widget,
            state,
            font_changed: true,
            font_size,
            cell_size: egui::Vec2::ZERO,
            quit_requested: false,
        }
    }

    /// Change font size for the next frame
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size.clamp(0.01, 10.0);
        self.font_changed = true;
    }

    /// Increment/decremet font size
    pub fn incr_font_size(&mut self, font_size: f32) {
        self.set_font_size(self.font_size + font_size)
    }

    /// Request closing of the application
    pub fn close(&mut self) {
        self.quit_requested = true;
    }
}

impl<W: RataguiWidget<State = S>, S> eframe::App for Ratagui<W, S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.quit_requested {
            self.quit_requested = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default()
            // NOTE very important, egui adds weird margin by default
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {

            let font_id = egui::TextStyle::Monospace.resolve(ui.style());

            // update font stuff
            if self.font_changed {
                // update finished
                self.font_changed = false;

                // set egui font size
                ctx.set_pixels_per_point(self.font_size);

                self.cell_size = ctx.fonts_mut(|f| egui::vec2(
                    f.glyph_width(&font_id, 'M'),
                    f.row_height(&font_id),
                ));
            }

            let cell_x = self.cell_size.x;
            let cell_y = self.cell_size.y;

            let available_size = ui.available_size();
            let cols = (available_size.x / cell_x).floor() as u16;
            let rows = (available_size.y / cell_y).floor() as u16;

            let padding_x = (available_size.x % cell_x) / 2.;
            let padding_y = (available_size.y % cell_y) / 2.;

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
                    max: egui::Pos2 { x: available_size.x, y: available_size.y }
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
                        x as f32 * cell_x + padding_x,
                        y as f32 * cell_y + padding_y,
                    );
                    let cell = buffer.cell(ratatui_core::layout::Position::new(x, y)).unwrap();

                    // draw bg
                    painter.rect_filled(
                        egui::Rect::from_min_size(pos, self.cell_size),
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
                for event in &i.events {
                    if !W::handle_raw_event(self, event.clone()) {
                        match conversion::convert_event(event, &i.modifiers, available_size, (rows, cols)) {
                            Some(crossterm_event) => W::handle_event(self, crossterm_event),
                            _ => {},
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
    font_size: Option<f32>,
    font: Option<&'static [u8]>,
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
            font_size.unwrap_or(1.0),
            font,
        )))),
    )
}

/// Same as `start` but with the less arguments
pub fn start_simple<W: RataguiWidget<State = S>, S>(app_name: &str, widget: W, state: S) -> eframe::Result<()> {
    start(app_name, widget, state, Theme::default(), None, None)
}
