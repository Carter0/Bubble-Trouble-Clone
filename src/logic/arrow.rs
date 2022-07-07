use crate::logic::player::Player;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use bevy::prelude::*;

pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_starting_arrow)
            .add_system(spawn_arrow)
            .add_system(grow_arrow);
    }
}

#[derive(Component)]
pub struct Arrow {
    // A destroyed arrow can be reset
    destroyed: bool,
}

// Spawns an invisible arrow in the beginning of the game
// so that there is always one arrow in the game
fn spawn_starting_arrow(mut commands: Commands) {
    let arrow_width = 10.0;
    let arrow_height = 30.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(arrow_width, arrow_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOWWIDTH, WINDOWHEIGHT, 1.0),
            ..Default::default()
        })
        .insert(Arrow { destroyed: true });
}

// Reset the arrow when the player presses up
// if the arrow has been destroyed
fn spawn_arrow(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut arrow_query: Query<(&mut Transform, &mut Arrow), (With<Arrow>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    let (mut arrow_transform, mut arrow) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    let spawn_position_x = player_transform.translation.x + 40.0;
    let spawn_position_y = -WINDOWHEIGHT / 2.0 + 40.0;

    let up = keyboard_input.just_pressed(KeyCode::W)
        || keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Space);

    if up && arrow.destroyed {
        arrow_transform.translation.x = spawn_position_x;
        arrow_transform.translation.y = spawn_position_y;
        arrow.destroyed = false;
    }
}

// Grow the arrow over time
fn grow_arrow(mut arrow_query: Query<(&mut Transform, &Arrow), With<Arrow>>) {
    let (mut transform, arrow) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    // A little hacky since it increases it on both directions when we only want the top
    if !arrow.destroyed {
        transform.scale.y += 0.3;
    }
}

// TODO
// fn arrow_collisions() {}
