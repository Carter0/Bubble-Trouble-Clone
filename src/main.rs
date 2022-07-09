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
        .add_startup_system(spawn_floor_and_walls)
        .run();
}

#[derive(Component)]
struct RightWall;

#[derive(Component)]
struct LeftWall;

#[derive(Component)]
struct Ceiling;

#[derive(Component)]
struct Floor;

// NOTE Origin point in the middle for all transforms
fn spawn_floor_and_walls(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    // The ceiling
    let ceiling_size_x = WINDOWWIDTH;
    let ceiling_size_y = 40.0;

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(ceiling_size_x, ceiling_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, WINDOWHEIGHT / 2.0, 1.0),
        ..Default::default()
    })
    .insert(Ceiling);

    // The floor
    let floor_size_x = WINDOWWIDTH;
    let floor_size_y = 40.0;

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(floor_size_x, floor_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0, 1.0),
        ..Default::default()
    })
    .insert(Floor);

    // The Left Wall
    let left_wall_size_x = 40.0;
    let left_wall_size_y = WINDOWHEIGHT;
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(left_wall_size_x, left_wall_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-WINDOWWIDTH / 2.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(LeftWall);

    // The Right Wall
    let right_wall_size_x = 40.0;
    let right_wall_size_y = WINDOWHEIGHT;
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(right_wall_size_x, right_wall_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOWWIDTH / 2.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(RightWall);
}
