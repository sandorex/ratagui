mod backend;
mod events;

use macroquad::{input::utils::{register_input_subscriber, repeat_all_miniquad_input}, prelude::*};
use ratatui::Terminal;
use crate::backend::MacroquadBackend;

fn main() {
    let conf = Conf {
        window_title: "Ratagui".to_string(),
        ..Default::default()
    };

    macroquad::Window::from_config(conf, macroquad_main());
}

struct TestHandler;

impl events::MacroquadEventHandler for TestHandler {
    fn handle_event(&mut self, event: crossterm::event::Event) {
        println!("got: {:?}", event);
    }
}

async fn macroquad_main() {
    let font = load_ttf_font("./3270NerdFont-Regular.ttf")
        .await
        .unwrap();

    let width = screen_width() as u16;
    let height = screen_height() as u16;

    let backend = MacroquadBackend::new(font, 20, width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    let render_target = render_target(width as u32, height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0., 0., 800., 600.));
    render_target_cam.render_target = Some(render_target.clone());

    let input_handler_id = register_input_subscriber();

    loop {
        // --- texture ---
        set_camera(&render_target_cam);

        terminal.draw(|f| {
            let size = f.area();
            let block = ratatui::widgets::Block::default()
                .title("Hello")
                .borders(ratatui::widgets::Borders::ALL);

            f.render_widget(block, size);
        }).unwrap();
        // --- texture ---

        set_default_camera();

        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );

        repeat_all_miniquad_input(&mut events::EventHandlerProxy(&mut TestHandler {}), input_handler_id);

        // draw_fps();
        next_frame().await;
    }
}
