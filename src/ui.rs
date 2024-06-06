use bevy::{app::AppExit, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (fps_update, button_interaction_system));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
struct FpsText;

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font: &Handle<Font> = &asset_server.load("fonts/Wellfleet-Regular.ttf");
    let font_size = 15.0;
    let font_color = Color::WHITE;

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
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
                    font: font.clone(),
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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: font_color,
                        },
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(85.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Info",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: font_color,
                        },
                    ));
                });
        });
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

fn button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        for child in children {
            let text = text_query.get_mut(*child).unwrap();

            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                    
                    match text.sections[0].value.as_str() {
                        "Quit" => {
                            exit.send(AppExit);
                        },
                        "Info" => {
                            println!("Info!");
                        }
                        _ => {
                            println!("Ooops, something went wrong!")
                        },
                    }

                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = NORMAL_BUTTON.into();
                }
            }
        };        
    }
}
