use bevy:: prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}

fn spawn_light(
    mut commands: Commands,
) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                intensity: 200_000.,
                range: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Light"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let floor = (
    //     PbrBundle {
    //         mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
    //         material: materials.add(Color::LIME_GREEN),
    //         ..default()
    //     },
    //     Collider::cuboid(7.5, 0.0, 7.5),
    //     Name::new("Floor"),
    // );

    commands
        .spawn(Collider::cuboid(10.0, 0.5, 10.0))
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(20.0, 1.0, 20.0))),
            material: materials.add(Color::LIME_GREEN),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        });
    //commands.spawn(floor);
}
