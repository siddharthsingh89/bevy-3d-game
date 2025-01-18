use crate::camera::CameraPlugin;
use crate::enemy_ship::EnemyShipPlugin;
use crate::world::WorldPlugin;
use agent::AgentPlugin;
use bevy::prelude::*;
use bevy_third_person_camera::*;
mod agent;
mod camera;
mod enemy_ship;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldPlugin,
            CameraPlugin,
            AgentPlugin,
            ThirdPersonCameraPlugin,
            EnemyShipPlugin,
        ))
        .run();
}
