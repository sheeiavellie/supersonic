use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_third_person_camera::*;

#[cfg(test)]
mod tests;

mod player;
mod camera;
mod world;
mod post_processing;
mod materials;
mod ui;

use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use post_processing::PostProcessPlugin;
use materials::DefinedMaterialsPlugin;
use ui::UIPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    app.add_plugins(
        RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        },
    );

    app.add_plugins((
        PlayerPlugin,
        CameraPlugin,
        WorldPlugin,
        ThirdPersonCameraPlugin,
        PostProcessPlugin,
        DefinedMaterialsPlugin,
        UIPlugin,
    ));

    app.run();
}
