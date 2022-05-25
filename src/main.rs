use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

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
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_floor_and_walls)
        .add_startup_system(spawn_ball)
        .add_system(player_movement)
        .add_system(spawn_bullets)
        .add_system(move_bullets)
        .add_system(despawn_bullets)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player(f32);

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // NOTE not really sure why gravity needs to be so big
    // But I think its dividing by the pixels per meter
    // TODO should I turn off gravity when the player is colliding with the floor?
    rapier_config.gravity = Vec2::new(0.0, -1000.0);
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
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0 + WINDOWHEIGHT / 20.0, 1.0),
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
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;

        let mut move_delta = Vec2::new(x_axis as f32, 0.0);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}

// Set the restitution coefficient and restitution combine rule
// when the collider is created.
// Restitution determines how bouncy the ball is.
fn spawn_ball(mut commands: Commands) {
    let circle_radius = 10.0;

    let circle = shapes::Circle {
        radius: circle_radius,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn()
        .insert(Collider::ball(circle_radius))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::ORANGE_RED),
                outline_mode: StrokeMode::new(Color::ORANGE_RED, 10.0),
            },
            Transform::default(),
        ));
}

#[derive(Component)]
struct Bullet;

fn spawn_bullets(
    keyboard_input: Res<Input<KeyCode>>,
    player_transform: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    let transform: &Transform = player_transform.single();
    let spawn_position_x = transform.translation.x;
    let spawn_position_y = transform.translation.y + 40.0;

    let bullet_size_width = 10.0;
    let bullet_size_height = 30.0;

    let up = keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up);

    // TODO might want to limit the total number of bullets that can be
    // on screen at any time
    if up {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(10.0, 70.0, 70.0),
                    custom_size: Some(Vec2::new(bullet_size_width, bullet_size_height)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(spawn_position_x, spawn_position_y, 1.0),
                ..Default::default()
            })
            .insert(Collider::cuboid(
                bullet_size_width / 2.0,
                bullet_size_height / 2.0,
            ))
            .insert(RigidBody::KinematicPositionBased)
            .insert(Bullet);
    }
}

// NOTE
// For position-based kinematic bodies, it is recommended to modify its Transform
// (changing its velocity won’t have any effect). This will let the physics
// engine compute the fictitious velocity of the kinematic body for more realistic
// intersections with other rigid-bodies.
fn move_bullets(mut bullet_positions_query: Query<&mut Transform, With<Bullet>>) {
    // TODO might want to make the position change relevative to the screen?
    for mut position in bullet_positions_query.iter_mut() {
        position.translation.y += 3.5;
    }
}

// If a bullet goes off screen destroy it
fn despawn_bullets(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in bullets_query.iter() {
        if transform.translation.y > WINDOWHEIGHT {
            commands.entity(entity).despawn();
            println!("Despawned the entity");
        }
    }
}

fn spawn_floor_and_walls(mut commands: Commands) {
    // The floor
    let floor_size_x = WINDOWWIDTH;
    let floor_size_y = 40.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(floor_size_x, floor_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(floor_size_x / 2.0, floor_size_y / 2.0));

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
        .insert(Collider::cuboid(
            left_wall_size_x / 2.0,
            left_wall_size_y / 2.0,
        ));

    // The Right Wall
    let right_wall_size_x = 40.0;
    let right_wall_size_y = WINDOWHEIGHT;
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(left_wall_size_x, left_wall_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOWWIDTH / 2.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(
            right_wall_size_x / 2.0,
            right_wall_size_y / 2.0,
        ));
}
