use bevy::prelude::*;

use crate::{
    camera::{update_post_processing, IsPostProcessingActive}, 
    post_processing::PostProcessSettings
};

#[test]
fn did_switch_camera_mode() {
    let mut app = App::new();

    app.add_systems(Update, update_post_processing);

    let camera_id = app.world
        .spawn((
            IsPostProcessingActive(false),
            PostProcessSettings {
                intensity: 0.0,
            },
        ))
        .id();

    assert!(app.world.get::<IsPostProcessingActive>(camera_id).is_some());
    
    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::KeyJ);
    app.insert_resource(input.clone());

    app.world.resource_mut::<ButtonInput<KeyCode>>().release(KeyCode::KeyJ);

    app.update();

    assert_eq!(app.world.get::<IsPostProcessingActive>(camera_id).unwrap().0, true);

    app.world.resource_mut::<ButtonInput<KeyCode>>().clear();

    app.update();

    input.press(KeyCode::KeyJ);
    app.insert_resource(input.clone());

    app.world.resource_mut::<ButtonInput<KeyCode>>().release(KeyCode::KeyJ);

    app.update();

    assert_eq!(app.world.get::<IsPostProcessingActive>(camera_id).unwrap().0, false);
}