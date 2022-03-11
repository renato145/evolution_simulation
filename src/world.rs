use macroquad::prelude::*;

pub struct World {}

impl World {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self) {
        loop {
            let t = get_time();
            clear_background(BLACK);
            draw_time(t);
            next_frame().await
        }
    }
}

fn draw_time(t: f64) {
    let text = format!("{:.1}s", t);
    let size = measure_text(&text, None, 20, 1.0);
    draw_text(
        &text,
        screen_width() - size.width - 5.0,
        15.0,
        20.0,
        LIGHTGRAY,
    );
}
