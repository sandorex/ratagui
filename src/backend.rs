//! Macroquad ratatui backend

use std::fmt::Display;
use macroquad::prelude::*;
use ratatui::buffer::Cell;

#[derive(Debug, Clone)]
pub enum MacroquadError {

}

// TODO
impl Display for MacroquadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?")?;

        Ok(())
    }
}

impl std::error::Error for MacroquadError {}

#[derive(Debug)]
pub struct MacroquadBackend {
    width: u16,
    height: u16,
    cell_width: f32,
    cell_height: f32,
    font: Font,
    cursor: (u16, u16),
    cursor_hidden: bool,
}

impl MacroquadBackend {
    pub fn new(font: Font, font_size: u16, width_px: u16, height_px: u16) -> Self {
        let dims = measure_text("W", Some(&font), font_size, 1.0);
        // dbg!(&dims);

        let cell_width = dims.width;
        let cell_height = dims.height;

        let width = (width_px as f32 / cell_width).floor() as u16;
        let height = (height_px as f32 / cell_height).floor() as u16;

        Self {
            width,
            height,
            cell_width,
            cell_height,
            font,
            cursor: (0, 0),
            cursor_hidden: true,
        }
    }
}

impl ratatui::backend::Backend for MacroquadBackend {
    type Error = std::io::Error;
    // type Error = MacroquadError;

    fn draw<'a, I>(&mut self, content: I) -> std::io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            let px = x as f32 * self.cell_width;
            let py = y as f32 * self.cell_height;

            // Background
            let bg = convert_color(cell.bg);
            draw_rectangle(px, py, self.cell_width, self.cell_height, bg);

            // Foreground
            if let Some(symbol) = cell.symbol().chars().next() {
                // let fg = BLACK;
                let fg = convert_color(cell.fg);

                draw_text_ex(
                    &symbol.to_string(),
                    px,
                    py + self.cell_height,
                    TextParams {
                        font: Some(&self.font),
                        // font_size: 22,
                        font_size: self.cell_height as u16,
                        color: fg,
                        ..Default::default()
                    },
                );
            }
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        self.cursor_hidden = true;
        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<ratatui::prelude::Position, Self::Error> {
        use ratatui::prelude::Position;
        Ok(Position::new(self.cursor.0, self.cursor.1))
    }

    fn set_cursor_position<P: Into<ratatui::prelude::Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        let pos = position.into();
        self.cursor = (pos.x, pos.y);

        Ok(())
    }

    fn clear(&mut self) -> std::io::Result<()> {
        clear_background(BLACK);
        Ok(())
    }

    // TODO do i need this?
    fn clear_region(&mut self, clear_type: ratatui::prelude::backend::ClearType) -> Result<(), Self::Error> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<ratatui::prelude::Size> {
        Ok(ratatui::prelude::Size::new(self.width, self.height))
    }

    fn window_size(&mut self) -> Result<ratatui::prelude::backend::WindowSize, Self::Error> {
        use ratatui::prelude::Size;
        use ratatui::prelude::backend::WindowSize;

        // NOTE as ratatui stores pixels as integer i am using ceil to make it more or equal to
        // actual pixel size
        Ok(WindowSize {
            columns_rows: Size::new(self.width, self.height),
            pixels: Size::new(
                (self.cell_width * Into::<f32>::into(self.width)).ceil() as u16,
                (self.cell_height * Into::<f32>::into(self.height)).ceil() as u16
            ),
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    // fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    //     where
    //         I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)> {
    //     todo!()
    // }
    //
    // fn hide_cursor(&mut self) -> Result<(), Self::Error> {
    //     todo!()
    // }
    //
    // fn show_cursor(&mut self) -> Result<(), Self::Error> {
    //     todo!()
    // }
    //
    // fn get_cursor_position(&mut self) -> Result<ratatui::prelude::Position, Self::Error> {
    //     todo!()
    // }
    //
    // fn set_cursor_position<P: Into<ratatui::prelude::Position>>(&mut self, position: P) -> Result<(), Self::Error> {
    //     todo!()
    // }
    //
    // fn clear(&mut self) -> Result<(), Self::Error> {
    //     todo!()
    // }
    //
    // fn clear_region(&mut self, clear_type: ratatui::prelude::backend::ClearType) -> Result<(), Self::Error> {
    //     todo!()
    // }
    //
    // fn size(&self) -> Result<ratatui::prelude::Size, Self::Error> {
    //     todo!()
    // }
    //
    // fn window_size(&mut self) -> Result<ratatui::prelude::backend::WindowSize, Self::Error> {
    //     todo!()
    // }
    //
    // fn flush(&mut self) -> Result<(), Self::Error> {
    //     todo!()
    // }
}

fn convert_color(color: ratatui::style::Color) -> Color {
    use ratatui::style::Color::*;

    match color {
        Reset => WHITE,
        Black => BLACK,
        Red => RED,
        Green => GREEN,
        Yellow => YELLOW,
        Blue => BLUE,
        Magenta => MAGENTA,
        Cyan => SKYBLUE,
        Gray => GRAY,
        DarkGray => DARKGRAY,
        LightRed => PINK,
        LightGreen => LIME,
        LightYellow => YELLOW,
        LightBlue => BLUE,
        LightMagenta => MAGENTA,
        LightCyan => SKYBLUE,
        White => WHITE,
        Rgb(r, g, b) => Color::from_rgba(r, g, b, 255),
        // TODO
        Indexed(_) => WHITE, // optionally implement 256 palette
    }
}
// impl From<ratatui::prelude::Color> for macroquad::color {
//     fn from(value: ratatui::prelude::Color) -> Self {
//         match value {
//             ratatui::style::Color::Reset
//         }
//     }
// }

