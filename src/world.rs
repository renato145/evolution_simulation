use crate::{
    food::{FoodController, FOOD_SIZE},
    slime::{SlimeController, SlimeState},
};
use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets, Skin},
};
use std::ops::Div;

pub struct World {
    food_controller: FoodController,
    slime_controller: SlimeController,
    simulation_speed: f32,
    time: f32,
}

impl World {
    pub fn new(initial_food: usize, initial_slimes: usize, food_limit: usize) -> Self {
        let mut food_controller = FoodController::new(10.0, food_limit, (5.0, 20.0), (0.5, 3.0));
        food_controller.spawn_n(initial_food);
        let mut slime_controller = SlimeController::new(1.8, 20.0, 0.05, 50.0, 300.0);
        slime_controller.spawn_n(initial_slimes);
        Self {
            food_controller,
            slime_controller,
            simulation_speed: 1.0,
            time: 0.0,
        }
    }

    pub async fn run(mut self) {
        setup_skin();
        loop {
            clear_background(BLACK);

            // Updates
            self.food_controller.set_time(self.time);
            self.slime_controller.set_time(self.time);

            for _ in 0..(self.simulation_speed.round() as usize) {
                self.food_controller.update_step();
                self.slime_controller
                    .update_step(&mut self.food_controller.population);
            }

            // Draws
            self.draw_food();
            self.draw_slimes();
            self.draw_status();
            self.draw_ui();
            next_frame().await;

            self.time += self.simulation_speed;
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

    /// Draws world status on top right corner of the screen
    fn draw_status(&self) {
        const FONT_SIZE: u16 = 25;
        let texts = [
            format!("Fps: {}s", get_fps()),
            format!("Time: {:.0}", self.time), // TODO: format number nicer
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

    fn draw_ui(&mut self) {
        widgets::Window::new(
            hash!(),
            vec2(25.0, screen_height() - 35.0 - 25.0),
            vec2(300.0, 35.0),
        )
        .label("Simulation speed")
        .ui(&mut *root_ui(), |ui| {
            ui.slider(hash!(), "[1 .. 30]", 1.0..30.0, &mut self.simulation_speed);
        });
    }
}

fn setup_skin() {
    let window_titlebar_style = root_ui().style_builder().font_size(20).build();
    let window_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(255, 255, 255, 180))
        .build();
    let label_style = root_ui().style_builder().build();
    let editbox_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(255, 255, 255, 200))
        .color_selected(Color::from_rgba(255, 255, 255, 255))
        .build();

    let ui_skin = Skin {
        window_titlebar_style,
        window_style,
        label_style,
        editbox_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);
}
