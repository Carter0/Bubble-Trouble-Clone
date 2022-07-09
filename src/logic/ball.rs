use crate::{Floor, LeftWall, RightWall};
use bevy::prelude::*;
pub struct BallPlugin;
use std::collections::HashMap;

const GRAVITY: f32 = -0.1;

#[derive(Default)]
struct BallMappings {
    ball_sizes: HashMap<u8, f32>,
    bounce_velocities: HashMap<u8, f32>,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BallMappings {
            ball_sizes: HashMap::from([(1, 20.0), (2, 40.0), (3, 60.0), (4, 80.0), (5, 100.0)]),
            bounce_velocities: HashMap::from([(1, 6.0), (2, 8.0), (3, 10.0), (4, 12.0), (5, 14.0)]),
        })
        .add_startup_system(spawn_starting_ball)
        .add_system(move_balls)
        .add_system(bounce_balls)
        .add_event::<PopBallEvent>()
        .add_system(pop_ball);
    }
}

#[derive(Component)]
pub struct Ball {
    y_velocity: f32,
    x_velocity: f32,
    // All balls should be squares
    // TODO im not sure I need this
    pub side: f32,
    // Range from 1-5.
    // The higher numbers would indicate a larger ball
    // If size is 1, then it is the smallest ball and can be destroyed if hit
    size: u8,
}

fn spawn_starting_ball(mut commands: Commands, ball_sizes_res: Res<BallMappings>) {
    let ball_size = 2;
    let ball_side = ball_sizes_res.ball_sizes[&2];

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(ball_side, ball_side)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Ball {
            y_velocity: 0.0,
            x_velocity: 1.0,
            side: ball_side,
            size: ball_size,
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
    ball_sizes_res: Res<BallMappings>,
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
        if ball_transform.translation.y - ball.side / 2.0 <= floor_transform.translation.y + 20.0 {
            // NOTE each size of ball only bounces a specific height
            let ball_bounce = ball_sizes_res.bounce_velocities[&ball.size];
            ball.y_velocity = ball_bounce;
        }

        if ball_transform.translation.x + ball.side / 2.0
            >= right_wall_transform.translation.x - 20.0
        {
            ball.x_velocity *= -1.0;
        }

        if ball_transform.translation.x - ball.side / 2.0
            <= left_wall_transform.translation.x + 20.0
        {
            ball.x_velocity *= -1.0;
        }
    }
}

pub struct PopBallEvent(pub Entity);

// When a ball gets popped spawn two smaller balls
// unless the ball is as small as it can go
fn pop_ball(
    mut pop_ball_event: EventReader<PopBallEvent>,
    mut commands: Commands,
    ball_query: Query<(&Ball, &Transform)>,
    ball_sizes_res: Res<BallMappings>,
) {
    for pop_ball_event in pop_ball_event.iter() {
        let ball_entity = pop_ball_event.0;

        if let Ok((ball, ball_transform)) = ball_query.get(ball_entity) {
            // if the ball can still get smaller
            if ball.size - 1 > 0 {
                let new_ball_size = ball.size - 1;
                let new_ball_side = ball_sizes_res.ball_sizes[&(new_ball_size)];

                // NOTE give each ball a 4 point lift so that
                // its possible they can bounce to the top of the screen
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(10.0, 70.0, 70.0),
                            custom_size: Some(Vec2::new(new_ball_side, new_ball_side)),
                            ..Default::default()
                        },
                        transform: *ball_transform,
                        ..Default::default()
                    })
                    .insert(Ball {
                        y_velocity: 4.0,
                        x_velocity: 1.0,
                        side: new_ball_side,
                        size: new_ball_size,
                    });

                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(10.0, 70.0, 70.0),
                            custom_size: Some(Vec2::new(new_ball_side, new_ball_side)),
                            ..Default::default()
                        },
                        transform: *ball_transform,
                        ..Default::default()
                    })
                    .insert(Ball {
                        y_velocity: 4.0,
                        x_velocity: -1.0,
                        side: new_ball_side,
                        size: new_ball_size,
                    });
            }
        }

        commands.entity(ball_entity).despawn();
    }
}
