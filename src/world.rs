use crate::food::{Food, FoodController};
use macroquad::prelude::*;

pub struct World {
    food_spawner: FoodController,
}

impl World {
    pub fn new(initial_food: usize) -> Self {
        let mut food_spawner = FoodController::new(0.2, 100, (1.0, 10.0), (0.5, 3.0));
        food_spawner.spawn_n(initial_food);
        Self { food_spawner }
    }

    pub async fn run(mut self) {
        self.food_spawner.reset_time();
        loop {
            clear_background(BLACK);

            // Updates
            self.food_spawner.update_food_positions();
            self.food_spawner.check_spawn();

            // Draws
            self.draw_status();
            draw_food(&self.food_spawner.population);
            next_frame().await
        }
    }

    /// Draws world status on top right corner of the screen
    fn draw_status(&self) {
        let texts = [
            format!("Time: {:.1}s", get_time()),
            format!("Food: {}", self.food_spawner.population.len()),
        ];
        let mut y = 15.0;
        for text in texts.iter() {
            let size = measure_text(&text, None, 20, 1.0);
            draw_text(text, screen_width() - size.width - 5.0, y, 20.0, LIGHTGRAY);
            y += size.height + 5.0;
        }
    }
}

fn draw_food(food: &[Food]) {
    food.iter()
        .for_each(|f| draw_circle(f.position.x, f.position.y, 2.0, GREEN));
}
