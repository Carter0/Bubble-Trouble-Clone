use crate::{Floor, LeftWall, RightWall};
use bevy::prelude::*;
pub struct BallPlugin;

const GRAVITY: f32 = -0.1;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_starting_ball)
            .add_system(move_balls)
            .add_system(bounce_balls);
    }
}

#[derive(Component)]
pub struct Ball {
    y_velocity: f32,
    x_velocity: f32,
}

fn spawn_starting_ball(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Ball {
            y_velocity: 0.0,
            x_velocity: 1.0,
        });
}

fn move_balls(mut ball_query: Query<(&mut Transform, &mut Ball)>) {
    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        ball.y_velocity += GRAVITY;
        ball_transform.translation.y += ball.y_velocity;
        ball_transform.translation.x += ball.x_velocity;
    }
}

fn bounce_balls(
    floor_query: Query<&Transform, (With<Floor>, Without<Ball>)>,
    right_wall_query: Query<&Transform, (With<RightWall>, Without<Ball>)>,
    left_wall_query: Query<&Transform, (With<LeftWall>, Without<Ball>)>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
) {
    let floor_transform = floor_query
        .get_single()
        .expect("Error: Could not find a single floor.");

    let right_wall_transform = right_wall_query
        .get_single()
        .expect("Error: Could not find a single right wall.");

    let left_wall_transform = left_wall_query
        .get_single()
        .expect("Error: Could not find a single left wall.");

    // NOTE ball is 40 by 40
    for (ball_transform, mut ball) in ball_query.iter_mut() {
        if ball_transform.translation.y - 20.0 <= floor_transform.translation.y + 20.0 {
            ball.y_velocity *= -1.0;
        }

        if ball_transform.translation.x + 20.0 >= right_wall_transform.translation.x - 20.0 {
            ball.x_velocity *= -1.0;
        }

        if ball_transform.translation.x - 20.0 <= left_wall_transform.translation.x + 20.0 {
            ball.x_velocity *= -1.0;
        }
    }
}
