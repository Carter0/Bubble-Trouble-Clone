use crate::{RightWall, LeftWall, WINDOWHEIGHT};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(player_wall_collisions);
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0 + 40.0, 1.0),
            ..Default::default()
        })
        .insert(Player { speed: 200.0 });
}

// Player can only move on the x axis
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    // NOTE this will crash when the player dies
    let (player, mut transform) = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    // Get input from the keyboard (WASD)
    let left: bool = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right: bool = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    // If left is pressed than it will be -1, right 1, both they cancel out.
    let x_axis: i8 = -(left as i8) + right as i8;

    // move the player
    let delta_time = time.delta_seconds();
    transform.translation.x += x_axis as f32 * player.speed * delta_time;
}

fn player_wall_collisions(
    right_wall_query: Query<&Transform, (With<RightWall>, Without<Player>)>,
    left_wall_query: Query<&Transform, (With<LeftWall>, Without<Player>)>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // TODO for now I just want to deal with the player colliding with the walls
    let mut player_transform = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    let right_wall_transform = right_wall_query
        .get_single()
        .expect("Error: Could not find a single right wall.");

    let left_wall_transform = left_wall_query
        .get_single()
        .expect("Error: Could not find a single left wall.");

    // NOTE Player is 40 by 40, Origin point in middle
    // Player collides with right wall
    if player_transform.translation.x + 20.0 >= right_wall_transform.translation.x - 20.0 {
        player_transform.translation.x = right_wall_transform.translation.x - 40.0;
    }

    // Player collides with left wall
    if player_transform.translation.x - 20.0 <= left_wall_transform.translation.x + 20.0 {
        player_transform.translation.x = left_wall_transform.translation.x + 40.0;
    }
}
