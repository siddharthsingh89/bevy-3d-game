use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}
fn uv_debug_texture() -> Image {
    use bevy::render::{render_asset::RenderAssetUsages, render_resource::*};
    const TEXTURE_SIZE: usize = 7;

    let mut palette = [
        164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153, 153,
        153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(12);
    }

    let mut img = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::MirrorRepeat,
        mag_filter: ImageFilterMode::Nearest,
        ..ImageSamplerDescriptor::linear()
    });
    img
}
fn setup_scene(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -0.7, -1.0), Vec3::X),
    ));
    // Sky
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::linear_rgb(0.1, 0.6, 1.0),
            ..default()
        })),
        Transform::default().with_scale(Vec3::splat(-4000.0)),
    ));
    // Ground
    let mut plane: Mesh = Plane3d::default().into();
    let uv_size = 4000.0;
    let uvs = vec![[uv_size, 0.0], [0.0, 0.0], [0.0, uv_size], [uv_size; 2]];
    plane.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    commands.spawn((
        Mesh3d(meshes.add(plane)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.65, 0.0).with_scale(Vec3::splat(80.)),
    ));
    spawn_trees(&mut meshes, &mut materials, &mut commands);
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

fn spawn_trees(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    commands: &mut Commands,
) {
    const N_TREES: usize = 100;
    let capsule = meshes.add(Capsule3d::default());
    let sphere = meshes.add(Sphere::default());
    let leaves = materials.add(Color::linear_rgb(0.0, 1.0, 0.0));
    let trunk = materials.add(Color::linear_rgb(0.4, 0.2, 0.2));

    let mut spawn_with_offset = |offset: f32| {
        for i in 0..N_TREES {
            let pos = race_track_pos(
                offset,
                (i as f32) / (N_TREES as f32) * std::f32::consts::PI * 2.0,
            );
            let [x, z] = pos.into();
            commands.spawn((
                Mesh3d(sphere.clone()),
                MeshMaterial3d(leaves.clone()),
                Transform::from_xyz(x, -0.3, z).with_scale(Vec3::splat(0.3)),
            ));
            commands.spawn((
                Mesh3d(capsule.clone()),
                MeshMaterial3d(trunk.clone()),
                Transform::from_xyz(x, -0.5, z).with_scale(Vec3::new(0.05, 0.3, 0.05)),
            ));
        }
    };
    spawn_with_offset(0.07);
    spawn_with_offset(-0.07);
}
