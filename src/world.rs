use bevy::{ pbr::ExtendedMaterial, prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::*;

use crate::materials::{Thermal, ThermalMaterialExtension};

/// Plugin responsible for World.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_light, spawn_floor, spawn_cubes))
            .add_systems(Update, rotate);
    }
}

// components
/// Describes an entity that rotates.
#[derive(Component)]
struct Rotates;

// systems
/// System that spawns debug cubes of different colors. One of them has `ExtendedMaterial` with `ThermalMaterialExtension`.
/// 
/// Should be removed later.
fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ext_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>>,
) {
    let thermal_render_layer = RenderLayers::layer(1);

    // cubes
    let cube1_position = Transform::from_xyz(2.0, 0.5, -7.0);
    let cube2_position = Transform::from_xyz(0.0, 0.5, -7.0);
    let cube3_position = Transform::from_xyz(-2.0, 0.5, -7.0);

    commands.spawn((
        MaterialMeshBundle  {
            mesh: meshes.add(Cuboid::default()),
            material: ext_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                },
                extension: ThermalMaterialExtension { 
                    temperature: 15.0,
                    intensity: 1.0,
                    is_infrared_mode_active: 0,
                },
            }),
            transform: cube1_position,
            ..default()
        },
        Thermal,
        Rotates,
        thermal_render_layer,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::GREEN),
            transform: cube2_position,
            ..default()
        },
        Rotates,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::BLUE),
            transform: cube3_position,
            ..default()
        },
        Rotates,
    ));
}

/// Spawns light.
/// 
/// It's important to mention, that light should be rendered in all layers. Use ` RenderLayers::all()`.
fn spawn_light(
    mut commands: Commands,
) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                intensity: 150_000_000.,
                range: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 75.0, 0.0),
            ..default()
        },
        Name::new("Light"),
        RenderLayers::all(),
    );

    commands.spawn(light);
}

/// Spawns floor.
/// 
/// Floor has a collider, so it should interact with physical objects.
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_dimensions = Vec3::new(100.0, 1.0, 100.0);
    let floor_position = Vec3::new(0.0, -1.0, 0.0);
    let floor_name = "Floor";

    commands
        .spawn(Collider::cuboid(floor_dimensions.x / 2.0, floor_dimensions.y / 2.0, floor_dimensions.z / 2.0))
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(floor_dimensions)),
            material: materials.add(Color::LIME_GREEN),
            transform: Transform::from_translation(floor_position),
            ..default()
        })
        .insert(Name::new(floor_name));
}

/// Describes the rotation of an entity.
fn rotate(
    time: Res<Time>, 
    mut query: Query<&mut Transform, With<Rotates>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}
