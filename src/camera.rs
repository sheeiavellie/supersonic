#[cfg(not(test))]
use bevy::pbr::ExtendedMaterial;
#[cfg(not(test))]
use crate::materials::{Thermal, ThermalMaterialExtension};

use bevy::{
    core_pipeline::core_3d::Camera3dDepthLoadOp, 
    prelude::*, 
    render::view::RenderLayers
};
use bevy_third_person_camera::{camera::Zoom, ThirdPersonCamera};

use crate::post_processing::PostProcessSettings;

/// Plugin for a Camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (sync_cameras, update_post_processing));
    }
}

// components
/// Describes whether the thermal post-processing is active or not.
#[derive(Component)]
pub struct IsPostProcessingActive(pub bool);

/// Describes the main camera on which post-processing should be applied.
#[derive(Component)]
pub struct MainCamera;

/// Describes the thermal camera which should show excluded from post-processing entities.
#[derive(Component)]
pub struct ThermalMaterialCamera;

// systems
/// System for spawning cameras. 
/// 
/// Note, that it uses `RenderLayers` and `Camera3dDepthLoadOp::Load` for `Camera3d`'s `depth_load_op`.
fn spawn_camera(
    mut commands: Commands,
) {
    let thermal_render_layer = RenderLayers::layer(1);

    // main camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },        
        ThirdPersonCamera {
            zoom: Zoom::new(1.0, 40.0),
            ..default()
        },
        PostProcessSettings {
            intensity: 0.0,
            ..default()
        },
        IsPostProcessingActive(false),
        MainCamera,
    ));

    // thermal material camera
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                depth_load_op: Camera3dDepthLoadOp::Load,
                ..default()
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThermalMaterialCamera,
        thermal_render_layer,
    ));
}

/// System that syncs `MainCamera` and `ThermalCamera` `Transform`s.
fn sync_cameras(
    main_query: Query<&Transform, (With<MainCamera>, Without<ThermalMaterialCamera>)>,
    mut thermal_query: Query<&mut Transform, (With<ThermalMaterialCamera>, Without<MainCamera>)>,
) {
    if let Ok(main_transform) = main_query.get_single() {
        if let Ok(mut thermal_transform) = thermal_query.get_single_mut() {
            *thermal_transform = *main_transform;
        }
    }
}

/// System that contains logic for switching visual modes.
/// 
/// It changes the intensity of `PostProcessSettings` in order to achive that.
/// 
/// Additionally it changes the `is_infrared_mode_active` of `ThermalMaterialExtension`.
/// 
/// Also keep in mind that it basicly switches material as a whole.
pub fn update_post_processing(
    mut settings: Query<(&mut PostProcessSettings, &mut IsPostProcessingActive)>,
    keys: Res<ButtonInput<KeyCode>>,

    #[cfg(not(test))]
    mut mat: Query<&mut Handle<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>, With<Thermal>>,
    #[cfg(not(test))]
    mut ext_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>>,
) {
    for (mut setting, mut is_active) in &mut settings {
        if keys.just_released(KeyCode::KeyJ) {
            match is_active.0 {
                true => {
                    is_active.0 = false;

                    setting.intensity = 0.0;

                    #[cfg(not(test))]
                    for mut material in mat.iter_mut() {
                        *material = ext_materials.add(ExtendedMaterial {
                            base: StandardMaterial {
                                base_color: Color::RED,
                                ..default()
                            },
                            extension: ThermalMaterialExtension { 
                                temperature: 15.0,
                                intensity: 1.0,
                                is_infrared_mode_active: 0,
                            },
                        });
                    }
                },
                false => {
                    is_active.0 = true;

                    setting.intensity = 1.0;

                    #[cfg(not(test))]
                    for mut material in mat.iter_mut() {
                        *material = ext_materials.add(ExtendedMaterial {
                            base: StandardMaterial {
                                base_color: Color::RED,
                                ..default()
                            },
                            extension: ThermalMaterialExtension { 
                                temperature: 15.0,
                                intensity: 1.0,
                                is_infrared_mode_active: 1,
                            },
                        });
                    }
                },
            }
        }
    }
}
