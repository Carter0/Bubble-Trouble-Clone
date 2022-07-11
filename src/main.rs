use bevy::prelude::*;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

mod logic;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bubble Trouble".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(logic::player::PlayerPlugin)
        .add_plugin(logic::arrow::ArrowPlugin)
        .add_plugin(logic::ball::BallPlugin)
        .add_plugin(logic::sides_of_screen::SidesOfScreenPlugin)
        .run();
}

