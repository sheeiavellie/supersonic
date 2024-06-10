//! [![](https://i.ibb.co/brHssHw/logo-full-small.png)](https://github.com/sheeiavellie/supersonic)
//! 
//! Blazingly fast drone simulator. Made with Bevy, Rapier and Rust.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_third_person_camera::*;

#[cfg(test)]
mod tests;

/// Drone models and Player logic.
pub mod player;
/// Camera logic.
pub mod camera;
/// All additional objects and their logic.
pub mod world;
/// Post-processing logic.
pub mod post_processing;
/// Contains all of the materials.
pub mod materials;
/// User iterface stuff.
pub mod ui;

use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use post_processing::PostProcessPlugin;
use materials::DefinedMaterialsPlugin;
use ui::UIPlugin;

/// Whole project entry point.
/// 
/// You should only initialize here the app itself and it's pulgins.  
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
