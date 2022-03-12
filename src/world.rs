use crate::{
    food::{FoodController, FOOD_SIZE},
    slime::{SlimeController, SlimeState},
};
use macroquad::prelude::*;
use std::ops::Div;

pub struct World {
    food_controller: FoodController,
    slime_controller: SlimeController,
}

impl World {
    pub fn new(initial_food: usize, initial_slimes: usize, food_limit: usize) -> Self {
        let mut food_controller = FoodController::new(0.1, food_limit, (5.0, 20.0), (0.5, 3.0));
        food_controller.spawn_n(initial_food);
        let mut slime_controller = SlimeController::new(1.8, 20.0, 0.05, 50.0, 5.0);
        slime_controller.spawn_n(initial_slimes);
        Self {
            food_controller,
            slime_controller,
        }
    }

    pub async fn run(mut self) {
        self.food_controller.reset_time();
        self.slime_controller.reset_time();
        loop {
            clear_background(BLACK);

            // Updates
            self.food_controller.update_step();
            self.slime_controller
                .update_step(&mut self.food_controller.population);

            // Draws
            self.draw_food();
            self.draw_slimes();
            self.draw_status();
            next_frame().await
        }
    }

    /// Draws world status on top right corner of the screen
    fn draw_status(&self) {
        const FONT_SIZE: u16 = 25;
        let texts = [
            format!("Fps: {}s", get_fps()),
            format!("Time: {:.1}s", get_time()),
            format!("Slimes: {}", self.slime_controller.population.len()),
            format!("Food: {}", self.food_controller.population.len()),
        ];
        let mut y = 15.0;
        for text in texts.iter() {
            let size = measure_text(text, None, FONT_SIZE, 1.0);
            draw_text(
                text,
                screen_width() - size.width - 5.0,
                y,
                FONT_SIZE as f32,
                LIGHTGRAY,
            );
            y += size.height + 5.0;
        }
    }

    fn draw_food(&self) {
        self.food_controller
            .population
            .iter()
            .for_each(|f| draw_circle(f.position.x, f.position.y, FOOD_SIZE, GREEN));
    }

    fn draw_slimes(&self) {
        let mouse = {
            let (x, y) = mouse_position();
            vec2(x, y)
        };
        self.slime_controller.population.iter().for_each(|slime| {
            let hovered = slime.is_point_inside(mouse, slime.size_vision());
            let color = if hovered {
                BLUE
            } else {
                match slime.state {
                    SlimeState::Normal => RED,
                    SlimeState::Jumping => LIME,
                    SlimeState::Breeding => PINK,
                }
            };
            draw_circle(slime.position.x, slime.position.y, slime.size(), color);
            if hovered {
                draw_circle_lines(
                    slime.position.x,
                    slime.position.y,
                    slime.size_vision(),
                    1.0,
                    YELLOW,
                );
                let text = format!("{:.0}", slime.energy());
                let size = measure_text(&text, None, 25, 1.0);
                draw_text(
                    &format!("{:.0}", slime.energy()),
                    slime.position.x - size.width.div(2.0),
                    slime.position.y - 10.0,
                    25.0,
                    WHITE,
                );
            }
        });
    }
}
