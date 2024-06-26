use std::f32::consts::PI;

use bevy::ecs::query;
use bevy::input::mouse::MouseWheel;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use map::{ CellType, Map, ResourceType };

mod map;

fn setup(
    mut commands: Commands,
    map: Res<Map>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    // Setting up a 3D camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.05,
            ..Default::default()
        }),
        ..Default::default()
    });

    for row in map.cells.iter() {
        for cell in row.iter() {
            // Calculate position in 3D space
            let position = Vec3::new(cell.x as f32, 0.0, cell.y as f32);

            let color = match cell.cell_types[0] {
                CellType::Empty => Color::BLACK,
                CellType::Resource(ResourceType::Water) => Color::BLUE,
                CellType::Resource(ResourceType::Field) => Color::GREEN,
                CellType::Resource(ResourceType::Tree) => Color::GREEN,
                CellType::Resource(ResourceType::Stone) => Color::GRAY,
                _ => Color::WHITE,
            };

            let cell_size = match cell.cell_types[0] {
                CellType::Empty => Vec3::new(1.0, 0.1, 1.0),
                CellType::Resource(ResourceType::Water) => Vec3::new(1.0, 0.1, 1.0),
                CellType::Resource(ResourceType::Field) => Vec3::new(1.0, 0.5, 1.0),
                CellType::Resource(ResourceType::Tree) => Vec3::new(0.5, 1.0, 0.5),
                CellType::Resource(ResourceType::Stone) => Vec3::new(1.0, 1.0, 1.0),
                _ => Vec3::new(1.0, 1.0, 1.0),
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(cell_size.x, cell_size.y, cell_size.z)),
                material: materials.add(color),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: (CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }).into(),
        ..default()
    });
}

fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>
) {
    let mut direction = Vec3::ZERO;
    let speed = 10.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        let delta = time.delta_seconds() * speed;
        for mut transform in query.iter_mut() {
            // Transform the direction by the camera's current rotation
            let rotated_direction = transform.rotation * direction;
            transform.translation += rotated_direction * delta;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::new(100, 100, None))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}
