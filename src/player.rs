use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Player;

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let camera = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error quering camera: {}", e)).unwrap()
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *camera.forward();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += *camera.right();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += *camera.left();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *camera.back();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(2.0, 2.0, 2.0))),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, 15.0, 0.0),
            ..default()
        },
        Player,
        Speed(2.0),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Restitution::coefficient(0.7),
        ThirdPersonCameraTarget,
        Name::new("Player")
    );

    commands.spawn(player);
}

// fn spawn_player(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
// ) {
//     let player = (
//         SceneBundle {
//             scene: assets.load("drone.glb#Scene0"),
//             transform: Transform::from_xyz(0.0, 0.5, 0.0)
//                 .with_scale(Vec3::new(0.1, 0.1, 0.1)),
//             ..default()
//         },
//         Player,
//         Speed(2.0),
//         ThirdPersonCameraTarget,
//     );

//     commands.spawn(player);
// }
