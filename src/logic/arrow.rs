use crate::logic::ball::{Ball, PopBallEvent};
use crate::logic::player::Player;
use crate::{WINDOWHEIGHT, WINDOWWIDTH};
use crate::logic::sides_of_screen::Ceiling;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const ARROWHEIGHT: f32 = 30.0;
const ARROWWIDTH: f32 = 10.0;
pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_starting_arrow)
            .add_system(spawn_arrow)
            .add_system(grow_arrow)
            .add_system(arrow_ceiling_collisions)
            .add_system(arrow_ball_collisions);
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
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(ARROWWIDTH, ARROWHEIGHT)),
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
    mut arrow_query: Query<(&mut Transform, &mut Arrow), Without<Player>>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    let (mut arrow_transform, mut arrow) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    let spawn_position_x = player_transform.translation.x;
    let spawn_position_y = -WINDOWHEIGHT / 2.0 + 40.0;

    let up = keyboard_input.just_pressed(KeyCode::W)
        || keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Space);

    if up && arrow.destroyed {
        arrow_transform.translation.x = spawn_position_x;
        arrow_transform.translation.y = spawn_position_y;
        arrow_transform.scale.y = 1.0;
        arrow.destroyed = false;
    }
}

// Grow the arrow over time
fn grow_arrow(mut arrow_query: Query<(&mut Transform, &Arrow), With<Arrow>>) {
    let (mut transform, arrow) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    // A little hacky since it increases it on both directions when we only want the top
    // TODO make variable depending on screen size
    if !arrow.destroyed {
        transform.scale.y += 0.5;
    }
}

// If the arrow collides with the ceiling it goes off screen
fn arrow_ceiling_collisions(
    mut arrow_query: Query<(&mut Arrow, &mut Transform), Without<Ceiling>>,
    ceiling_query: Query<&Transform, With<Ceiling>>,
) {
    let ceiling_transform = ceiling_query
        .get_single()
        .expect("Error: Could not find a single ceiling.");

    let (mut arrow, mut arrow_transform) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    // NOTE
    // Because the way we are growing the arrow is by scaling it.
    // We calculate the top of the arrow by multiplying it by the scale
    // and dividing a bunch because the origin point of everything is always
    // in the middle
    if arrow_transform.scale.y * ARROWHEIGHT / 4.0 >= ceiling_transform.translation.y - 20.0 {
        arrow.destroyed = true;
        arrow_transform.translation.x = WINDOWWIDTH * 2.0;
    }
}

// If the arrow collides with the ball it goes off screen and the ball gets popped
fn arrow_ball_collisions(
    mut arrow_query: Query<(&mut Arrow, &mut Transform), Without<Ball>>,
    ball_query: Query<(&Transform, &Ball, Entity), With<Ball>>,
    mut pop_ball_event: EventWriter<PopBallEvent>,
) {
    let (mut arrow, mut arrow_transform) = arrow_query
        .get_single_mut()
        .expect("Error: Could not find a single arrow.");

    let arrow_size_x = arrow_transform.scale.x * ARROWWIDTH;
    let arrow_size_y = arrow_transform.scale.y * ARROWHEIGHT;
    for (ball_transform, ball, ball_entity) in ball_query.iter() {
        if let Some(_collision) = collide(
            ball_transform.translation,
            Vec2::new(ball.side, ball.side),
            arrow_transform.translation,
            Vec2::new(arrow_size_x, arrow_size_y),
        ) {
            arrow.destroyed = true;
            arrow_transform.translation.x = WINDOWWIDTH * 2.0;
            pop_ball_event.send(PopBallEvent(ball_entity));
        }
    }
}
