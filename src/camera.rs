use bevy::{
    core_pipeline::core_3d::Camera3dDepthLoadOp, 
    pbr::ExtendedMaterial, prelude::*, 
    render::view::RenderLayers
};


use crate::{
    materials::{Thermal, ThermalMaterialExtension}, 
    post_processing::PostProcessSettings
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (sync_cameras, update_post_processing));
    }
}

// components
#[derive(Component)]
struct IsPostProcessingActive(bool);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct ThermalMaterialCamera;

// systems
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
        // ThirdPersonCamera {
        //     zoom: Zoom::new(1.0, 40.0),
        //     ..default()
        // },
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

fn update_post_processing(
    mut settings: Query<(&mut PostProcessSettings, &mut IsPostProcessingActive)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mat: Query<&mut Handle<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>, With<Thermal>>,
    mut ext_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>>,
) {
    for (mut setting, mut is_active) in &mut settings {
        if keys.just_released(KeyCode::KeyJ) {
            match is_active.0 {
                true => {
                    is_active.0 = false;

                    setting.intensity = 0.0;

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

                    // if let Ok(mut ma) = mat.get_single_mut() {
                    //     *ma = ext_materials.add(ExtendedMaterial {
                    //         base: StandardMaterial {
                    //             base_color: Color::RED,
                    //             ..default()
                    //         },
                    //         extension: ThermalMaterialExtension { 
                    //             temperature: 15.0,
                    //             intensity: 1.0,
                    //             is_infrared_mode_active: 1,
                    //         },
                    //     });
                    // }
                },
            }
        }
    }
}
