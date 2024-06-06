use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, fps_update);
    }
}

#[derive(Component)]
struct FpsText;

fn setup_ui(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    let font_size = 15.0;
    let font_color = Color::WHITE;

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/Wellfleet-Regular.ttf"),
                    font_size: font_size,
                    color: font_color,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: font_size,
                    color: font_color,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/Wellfleet-Regular.ttf"),
                    font_size: font_size,
                    color: font_color,
                }
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}

fn fps_update(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
