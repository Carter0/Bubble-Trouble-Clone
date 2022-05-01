use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOWHEIGHT: f32 = 1200.0;
const WINDOWWIDTH: f32 = 1500.0;

// NOTE the units are pixels for pretty much the whole thing, rapier now deals with the conversion to meters in the backend

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bubble Trouble".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_floor_and_walls)
        .add_system(player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player(f32);

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size = 40.0;

    // Spawn entity with `Player` struct as a component for access in movement query.
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0))
        .insert(Player(300.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}

fn spawn_floor_and_walls(mut commands: Commands) {
    let sprite_size_x = WINDOWWIDTH;
    let sprite_size_y = 40.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(sprite_size_x, sprite_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -500.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(sprite_size_x / 2.0, sprite_size_y / 2.0));
}
