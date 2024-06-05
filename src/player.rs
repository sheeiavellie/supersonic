use bevy::{pbr::ExtendedMaterial, prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::*;

use crate::materials::{Thermal, ThermalMaterialExtension};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut ExternalImpulse, With<Player>>,
) {
    for mut ext_imp in controllers.iter_mut() {
        if keys.pressed(KeyCode::ArrowUp) {
            ext_imp.impulse = Vec3::new(0.0, 5.0, 0.0);
        }
        if keys.pressed(KeyCode::KeyJ) {
            
        }
        if keys.pressed(KeyCode::ArrowDown) {
            ext_imp.impulse = -Vec3::new(0.0, 5.0, 0.0);
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ext_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>>,
) {
    let thermal_render_layer = RenderLayers::layer(1);

    let player_dimensions = Vec3::new(2.5, 1.0, 3.0);
    let player_position = Vec3::new(0.0, 10.0, 0.0);
    let player_name = "Player";

    let player = (
        MaterialMeshBundle {
            mesh: meshes.add(Cuboid::from_size(player_dimensions)),
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
            transform: Transform::from_xyz(player_position.x, player_position.y, player_position.z),
            ..default()
        },
        Thermal,
        Player,
        RigidBody::Dynamic,
        GravityScale(1.0),
        ExternalForce {
            force: Vec3::new(0.0, 73.8, 0.0),
            ..default()
        },
        ExternalImpulse {
            ..default()
        },
        Collider::cuboid(player_dimensions.x / 2.0, player_dimensions.y / 2.0, player_dimensions.z / 2.0),
        //ThirdPersonCameraTarget,
        Name::new(player_name),
        thermal_render_layer,
    );

    commands.spawn(player);
}
