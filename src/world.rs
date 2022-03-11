use crate::food::{Food, FoodController};
use macroquad::prelude::*;

pub struct World {
    food_spawner: FoodController,
}

impl World {
    pub fn new(initial_food: usize) -> Self {
        let mut food_spawner = FoodController::new(0.2, 100, (1, 10), (0.5, 3.0));
        food_spawner.spawn_n(initial_food);
        Self { food_spawner }
    }

    pub async fn run(mut self) {
        self.food_spawner.reset_time();
        loop {
            let t = get_time();
            clear_background(BLACK);

            // Updates
            self.food_spawner.update_food_positions();
            self.food_spawner.check_spawn();

            // Draws
            draw_time(t);
            draw_food(&self.food_spawner.population);
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

fn draw_food(food: &[Food]) {
    food.iter()
        .for_each(|f| draw_circle(f.position.x, f.position.y, 5.0, GREEN));
}
