#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use entities::EntitiesPlugin;
use interactions::InteractionsPlugin;
use ui::SimulationUIPlugin;

mod entities;
mod interactions;
mod ui;
mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Evolution simulation".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(EntitiesPlugin)
        .add_plugin(InteractionsPlugin)
        .add_plugin(SimulationUIPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
