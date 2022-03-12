use crate::{food::FoodController, slime::SlimeController};
use macroquad::prelude::*;

pub struct World {
    food_controller: FoodController,
    slime_controller: SlimeController,
}

impl World {
    pub fn new(initial_food: usize, initial_slimes: usize) -> Self {
        let mut food_controller = FoodController::new(0.2, 100, (1.0, 10.0), (0.5, 3.0));
        food_controller.spawn_n(initial_food);
        let mut slime_controller = SlimeController::new(1.5, 10.0, 5.0);
        slime_controller.spawn_n(initial_slimes);
        Self {
            food_controller,
            slime_controller,
        }
    }

    pub async fn run(mut self) {
        self.food_controller.reset_time();
        loop {
            clear_background(BLACK);

            // Updates
            self.food_controller.update_food_positions();
            self.food_controller.check_spawn();

            // Draws
            self.draw_status();
            self.draw_food();
            self.draw_slimes();
            next_frame().await
        }
    }

    /// Draws world status on top right corner of the screen
    fn draw_status(&self) {
        let texts = [
            format!("Time: {:.1}s", get_time()),
            format!("Food: {}", self.food_controller.population.len()),
        ];
        let mut y = 15.0;
        for text in texts.iter() {
            let size = measure_text(&text, None, 20, 1.0);
            draw_text(text, screen_width() - size.width - 5.0, y, 20.0, LIGHTGRAY);
            y += size.height + 5.0;
        }
    }

    fn draw_food(&self) {
        self.food_controller
            .population
            .iter()
            .for_each(|f| draw_circle(f.position.x, f.position.y, 2.0, GREEN));
    }

    fn draw_slimes(&self) {
        self.slime_controller
            .population
            .iter()
            .for_each(|f| draw_circle(f.position.x, f.position.y, f.size(), RED));
    }
}
