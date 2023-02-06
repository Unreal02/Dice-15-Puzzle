use crate::{
    buffered_input::{InputInversionFlag, MoveImmediate},
    player::PlayerState,
    ui::*,
};

#[derive(Component)]
pub struct GameUI;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_game_ui)
            .add_system(game_ui_button_system)
            .add_system(game_ui_text_system)
            .add_system_set(SystemSet::on_enter(PlayerState::Clear).with_system(spawn_clear_ui))
            .add_system_set(SystemSet::on_exit(PlayerState::Clear).with_system(despawn_clear_ui));
    }
}

fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input_system: Query<(&InputInversionFlag, &MoveImmediate)>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let (input_inversion, move_immediate) = input_system.single();
    let button_toggle_on_image = UiImage::from(asset_server.load("images/button_toggle_on.png"));
    let button_toggle_off_image = UiImage::from(asset_server.load("images/button_toggle_off.png"));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("GAME_UI"),
            GameUI,
        ))
        .with_children(|parent| {
            // reset button
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Reset,
                asset_server.load("images/button_reset.png").into(),
            );

            // shuffle button
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(170.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Shuffle,
                asset_server.load("images/button_shuffle.png").into(),
            );

            // animation toggle button
            spawn_toggle_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Animation".to_string(),
                font.clone(),
                MyButtonType::AnimationToggle,
                match move_immediate.0 {
                    true => button_toggle_off_image.clone(),
                    false => button_toggle_on_image.clone(),
                },
            );

            // input inversion button
            spawn_toggle_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(115.0),
                    ..default()
                },
                "Input Inversion".to_string(),
                font.clone(),
                MyButtonType::InputInversion,
                match input_inversion.0 {
                    true => button_toggle_on_image.clone(),
                    false => button_toggle_off_image.clone(),
                },
            );

            // mode selection button
            spawn_button(
                parent,
                UiRect {
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Mode\nPractice".to_string(),
                font.clone(),
                MyButtonType::ModeSelection,
                Some(MyTextType::ModeSelection),
                asset_server.load("images/button.png").into(),
            );

            // share button
            spawn_image_button(
                parent,
                UiRect {
                    left: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Share,
                asset_server.load("images/button_share.png").into(),
            );

            // player info
            parent.spawn((
                TextBundle::from_section(
                    "Time: 00:00.00\nMoves: 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(50.0),
                        top: Val::Px(175.0),
                        ..default()
                    },
                    ..default()
                }),
                MyTextType::PlayerInfo,
            ));
        });
}

fn spawn_clear_ui(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    commands.entity(game_ui.single()).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Clear!!",
                TextStyle {
                    font: font.clone(),
                    font_size: TEXT_SIZE,
                    color: Color::BLACK,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(70.0),
                    ..default()
                },
                ..default()
            }),
            MyTextType::GameClear,
        ));
    });
}

fn despawn_clear_ui(mut commands: Commands, my_ui_query: Query<(Entity, &MyTextType)>) {
    for (ui, &ui_type) in &my_ui_query {
        if ui_type == MyTextType::GameClear {
            commands.entity(ui).despawn_recursive();
        }
    }
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtonType,
    text_type: Option<MyTextType>,
    image: UiImage,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position,
                    ..default()
                },
                image,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            let bundle = TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: TEXT_SIZE,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER);
            match text_type {
                Some(text_type) => parent.spawn((bundle, text_type)),
                None => parent.spawn(bundle),
            };
        });
}

pub fn spawn_image_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    button_type: MyButtonType,
    image: UiImage,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            image,
            ..default()
        },
        button_type,
    ));
}

fn spawn_toggle_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtonType,
    image: UiImage,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                    position_type: PositionType::Absolute,
                    position,
                    ..default()
                },
                image,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(50.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        });
}
