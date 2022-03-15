use crate::entities::{food::FoodCount, slime::SlimeCount};
use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct SimulationUIPlugin;

impl Plugin for SimulationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(show_stats)
            .add_system(show_ui);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Show stats
    let style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Slimes: ".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "\nFoods: ".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style,
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn show_stats(
    slime_count: Res<SlimeCount>,
    food_count: Res<FoodCount>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", slime_count.0);
    text.sections[3].value = format!("{}", food_count.0);
}

fn show_ui(mut _egui_context: ResMut<EguiContext>) {
    // TODO
    // egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
    //     ui.label("world");
    // });
}
