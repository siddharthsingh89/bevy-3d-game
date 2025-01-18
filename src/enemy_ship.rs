use bevy::prelude::*;

pub struct EnemyShipPlugin;

impl Plugin for EnemyShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship)
            .add_systems(Update, move_ship);
    }
}

#[derive(Component)]
struct EnemyShip;

#[derive(Component)]
struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 25.0,
            speed: 2.0,
        }
    }
}

fn race_track_pos(offset: f32, t: f32) -> Vec2 {
    let x_tweak = 2.0;
    let y_tweak = 3.0;
    let scale = 8.0;
    let x0 = ops::sin(x_tweak * t);
    let y0 = ops::cos(y_tweak * t);
    let dx = x_tweak * ops::cos(x_tweak * t);
    let dy = y_tweak * -ops::sin(y_tweak * t);
    let dl = ops::hypot(dx, dy);
    let x = x0 + offset * dy / dl;
    let y = y0 - offset * dx / dl;
    Vec2::new(x, y) * scale
}

fn move_ship(mut cubes: Query<(&mut Transform, &mut Movable)>, time: Res<Time>) {
    for (mut transform, mut cube) in &mut cubes {
        let time = time.elapsed_secs() * 0.25;
        let t = time + 0.5 * cube.speed;
        let dx = ops::cos(t);
        let dz = -ops::sin(3.0 * t);
        let speed_variation = (dx * dx + dz * dz).sqrt() * 0.15;
        let t = t + speed_variation;
        let prev = transform.translation;
        transform.translation.x = race_track_pos(0.0, t).x;
        transform.translation.z = race_track_pos(0.0, t).y;
        transform.translation.y = 1.59;
        let delta = transform.translation - prev;
        transform.look_to(delta, Vec3::Y);
    }
}

fn spawn_ship(mut commands: Commands, assets: Res<AssetServer>) {
    let entity_spawn = Vec3::ZERO;
    let ship = (
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("craft_speederD.gltf"))),
        Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Movable::new(entity_spawn),
    );
    commands.spawn(ship);
}
