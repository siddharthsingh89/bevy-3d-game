use bevy::prelude::*;
use bevy_third_person_camera::*;
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_agent)
            .add_systems(Update, agent_movement);
    }
}

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Speed(f32);

fn agent_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Agent>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Agent>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_secs();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
fn spawn_agent(mut commands: Commands, assets: Res<AssetServer>) {
    let agent = (
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("alien.glb"))),
        Transform::from_xyz(-2.0, -0.70, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Agent,
        ThirdPersonCameraTarget,
        Speed(2.5),
    );
    commands.spawn(agent);
}
