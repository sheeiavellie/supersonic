use bevy::prelude::*;
use bevy_third_person_camera::{camera::Zoom, *};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(1.0, 40.0),
            ..default()
        },
    );

    commands.spawn(camera);
}
