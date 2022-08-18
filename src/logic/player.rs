use crate::logic::ball::Ball;
use crate::logic::sides_of_screen::{LeftWall, RightWall};
use crate::WINDOWHEIGHT;
use bevy::sprite::collide_aabb::collide;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(load_scene_system)
            .add_system(test)
            .add_system(move_player)
            .add_system(player_ball_collisions)
            .add_system(player_wall_collisions);
    }
}

#[derive(Component, Reflect)]
pub struct Player {
    speed: f32,
}

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn_bundle(SceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load("scenes/test.scn.ron"),
        ..default()
    });
}

fn test(player_query: Query<&Player>) {
    for player in player_query.iter() {
        println!("{}", player.speed);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                // TODO This should be a global and also depend on screen size probs
                // Then again this changes with the sprite so who knows lol
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0 + 40.0, 1.0),
            ..Default::default()
        })
        // TODO should be a percentage of the screen
        .insert(Player { speed: 300.0 });
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

fn player_ball_collisions(
    player_query: Query<(&Transform, Entity), With<Player>>,
    ball_query: Query<(&Transform, &Ball), Without<Player>>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    for (ball_transform, ball) in ball_query.iter() {
        if let Some(_collision) = collide(
            ball_transform.translation,
            Vec2::new(ball.side, ball.side),
            player_transform.translation,
            Vec2::new(40.0, 40.0),
        ) {
            // TODO
            // Add a restart key or a menu or something
            // This will break the game because our code relies on a player existing
            // Eventually will need lives
            commands.entity(player_entity).despawn();
        }
    }
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
