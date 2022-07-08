use bevy::prelude::*;
pub struct BallPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(spawn_starting_arrow)
    }
}

#[derive(Component)]
pub struct Ball;

fn spawn_ball(mut commands: Commands) {

}
