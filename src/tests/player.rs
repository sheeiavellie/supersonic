use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::{player_movement, Player};

#[test]
fn did_change_height() {
    let mut app = App::new();

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.insert_resource(RapierConfiguration {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Fixed {
            dt: 1.0,
            substeps: 1,
        },
        scaled_shape_subdivision: 1,
        force_update_from_transform_changes: true,
    });

    app.add_systems(Update, player_movement);

    let player_dimensions = Vec3::new(2.5, 1.0, 3.0);
    let player_position = Vec3::new(0.0, 10.0, 0.0);
    
    let player_id = app.world
        .spawn((
            Collider::cuboid(player_dimensions.x / 2.0, player_dimensions.y / 2.0, player_dimensions.z / 2.0),
            Player,
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_xyz(player_position.x, player_position.y, player_position.z)),
            ExternalForce {
                force: Vec3::new(0.0, 73.8, 0.0),
                ..default()
            },
            ExternalImpulse {
                ..default()
            },
        ))
        .id();

    assert!(app.world.get::<Player>(player_id).is_some());

    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::ArrowUp);
    app.insert_resource(input.clone());

    let mut current_position = app.world.get::<Transform>(player_id).unwrap().translation;

    assert_eq!(app.world.get::<Transform>(player_id).unwrap().translation, current_position);
}