mod backend;

use macroquad::prelude::*;
use ratatui::Terminal;
use crate::backend::MacroquadBackend;

fn main() {
    let conf = Conf {
        window_title: "Ratatuvy".to_string(),
        ..Default::default()
    };

    macroquad::Window::from_config(conf, macroquad_main());
}

async fn macroquad_main() {
    let font = load_ttf_font("./3270NerdFont-Regular.ttf")
        .await
        .unwrap();

    let width = screen_width() as u16;
    let height = screen_height() as u16;
    // let rect = Rect {
    //     x: 0.,
    //     y: 0.,
    //     w: width as f32,
    //     h: height as f32,
    // };

    let backend = MacroquadBackend::new(font, 20, width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    let render_target = render_target(width as u32, height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0., 0., 800., 600.));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        // --- texture ---
        set_camera(
            // &Camera2D::from_display_rect(
            //     Rect { x: 0., y: 0., w: width as f32, h: height as f32 }
            // )
            // &Camera2D {
            //     target: vec2(rect.x + rect.w / 2., rect.y + rect.h / 2.),
            //     zoom:  vec2(1. / rect.w * 2., -1. / rect.h * 2.),
            //     // target: vec2(100., 100.),
            //     render_target: Some(render_target.clone()),
            //     ..Default::default()
            // }
            &render_target_cam
        );
        // set_camera(&Camera2D {
        //     // zoom: vec2(1./width as f32, 1./height as f32),
        //     zoom: vec2(0.01, 0.01),
        //     target: vec2(100., 100.),
        //     // target: vec2(1./(width as f32 / 2.), 1./(height as f32 / 2.)),
        //     // zoom: vec2(2., 2.),
        //     // offset: vec2(-1., -1.),
        //     render_target: Some(render_target.clone()),
        //     ..Default::default()
        // });

        // clear_background(GRAY);

        // let p = render_target_cam.world_to_screen(vec2(100., 100.));
        // draw_rectangle(0., 0., p.x, p.y, RED);
        // draw_rectangle(0., 0., 100., 100., BLUE);

        // terminal.autoresize();

        terminal.draw(|f| {
            let size = f.area();
            let block = ratatui::widgets::Block::default()
                .title("Hello")
                .borders(ratatui::widgets::Borders::ALL);

            // width: 123, -92
            // height: 75, -47
            f.render_widget(block, size);
            // f.render_widget(block, ratatui::layout::Rect { x: 0, y: 0, width: 31, height: 25 });
        }).unwrap();
        // --- texture ---

        set_default_camera();

        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            // screen_width() / 2.0,
            // screen_height() / 2.0,
            WHITE,
            DrawTextureParams {
                // dest_size: Some(vec2(width as f32, height as f32)),
                // dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        // draw_fps();
        next_frame().await;
    }

    // let mut angle = 0.0;
    //
    // loop {
    //     clear_background(BLACK);
    //
    //     draw_text_ex("Custom font size:", 20.0, 20.0, TextParams::default());
    //     let mut y = 20.0;
    //
    //     for font_size in (30..100).step_by(20) {
    //         let text = "abcdef";
    //         let params = TextParams {
    //             font_size,
    //             ..Default::default()
    //         };
    //
    //         y += font_size as f32;
    //         draw_text_ex(text, 20.0, y, params);
    //     }
    //
    //     draw_text_ex("Dynamic font scale:", 20.0, 400.0, TextParams::default());
    //     draw_text_ex(
    //         "abcd",
    //         20.0,
    //         450.0,
    //         TextParams {
    //             font_size: 50,
    //             font_scale: get_time().sin() as f32 / 2.0 + 1.0,
    //             ..Default::default()
    //         },
    //     );
    //
    //     draw_text_ex("Custom font:", 400.0, 20.0, TextParams::default());
    //     draw_text_ex(
    //         "abcd",
    //         400.0,
    //         70.0,
    //         TextParams {
    //             font_size: 50,
    //             font: Some(&font),
    //             ..Default::default()
    //         },
    //     );
    //
    //     draw_text_ex(
    //         "abcd",
    //         400.0,
    //         160.0,
    //         TextParams {
    //             font_size: 100,
    //             font: Some(&font),
    //             ..Default::default()
    //         },
    //     );
    //
    //     draw_text_ex(
    //         "abcd",
    //         screen_width() / 4.0 * 2.0,
    //         screen_height() / 3.0 * 2.0,
    //         TextParams {
    //             font_size: 70,
    //             font: Some(&font),
    //             rotation: angle,
    //             ..Default::default()
    //         },
    //     );
    //
    //     let center = get_text_center("abcd", Option::None, 70, 1.0, angle * 2.0);
    //     draw_text_ex(
    //         "abcd",
    //         screen_width() / 4.0 * 3.0 - center.x,
    //         screen_height() / 3.0 * 2.0 - center.y,
    //         TextParams {
    //             font_size: 70,
    //             rotation: angle * 2.0,
    //             ..Default::default()
    //         },
    //     );
    //
    //     angle -= 0.030;
    //
    //     next_frame().await
    // }
}
