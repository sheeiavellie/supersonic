use bevy::{app::AppExit, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, ui::FocusPolicy};

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

const DIALOG_MENU_BACKGROUND: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct DialogMenu;

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

fn despawn_dialog_menu(
    commands: &mut Commands,
    dialog_menu_query: &Query<Entity, With<DialogMenu>>,
) {
    if let Ok(dialog_menu_entity) = dialog_menu_query.get_single() {
        commands.entity(dialog_menu_entity).despawn_recursive();
    }
}

fn build_dialog_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let font: &Handle<Font> = &asset_server.load("fonts/Wellfleet-Regular.ttf");
    let font_size = 15.0;
    let font_color = Color::WHITE;

    commands
        .spawn((
            NodeBundle {
                focus_policy: FocusPolicy::Block,
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(185.0),
                    right: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: DIALOG_MENU_BACKGROUND.into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
            DialogMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(10.0),
                            height: Val::Px(10.0),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            top: Val::Px(0.5),
                            right: Val::Px(1.0),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "x",
                        TextStyle {
                            font_size: 10.0,
                            ..default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new(
                            "Controls:\n^ ArrowUp for Up\nv ArrowDown for Down\n[ 0¯] J to switch camera mode\n\n",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: font_color,
                            },
                        ),
                        TextSection::new(
                            "Created by:\nAlexander V. Trotsky",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: font_color,
                            },
                        ),
                    ])
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    })
                );
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
    dialog_menu_query: Query<Entity, With<DialogMenu>>,
    mut exit: EventWriter<AppExit>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                            if let Err(_err) = dialog_menu_query.get_single() {
                                build_dialog_menu(&mut commands, &asset_server);
                            }
                        },
                        "x" => {
                            if let Ok(_dialog_open_flag) = dialog_menu_query.get_single() {
                                despawn_dialog_menu(&mut commands, &dialog_menu_query);
                            }
                        },
                        _ => {
                            println!("Ooops, something went wrong!");
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
