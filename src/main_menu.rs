use bevy::{prelude::*};

use crate::AppState;

pub struct MainMenuPlugin;

#[derive(Component)]
struct UiText;

impl Plugin for MainMenuPlugin
{
    fn build(&self, app: &mut App){
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system_set(SystemSet::on_enter(AppState::MainMenu)
            .with_system(setup_main_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu)
            .with_system(key_press))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu)
            .with_system(destruct_main_menu));
            
        app.add_system_set(SystemSet::on_update(AppState::GameSetup)
            .with_system(setup_setupgame));

        app.add_system_set(SystemSet::on_update(AppState::GameDestruct)
            .with_system(destruct_game));
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    let font_handle: Handle<Font> = asset_server.load("fonts\\Roboto-Regular.ttf");
    let game_name: &str = "Apocalypse Farmer";
    let game_start_msg: &str = "Press space to begin game!";

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Color::rgb(0.0, 0.0, 0.0).into(),
        ..Default::default()
    })
    .insert(UiText)
    // The container where the text is placed
    .with_children(|parent| {
            //Name of the game
        parent.spawn_bundle(TextBundle {
            style: Style {
                // Set height to font size * number of text lines
                size: Size::new(Val::Auto, Val::Px(62. * 1.)),
                // Set left margin to auto to push the text to the right
                margin: UiRect {
                    left: Val::Auto,
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: game_name.to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.1, 0.1)
                        }
                    },

                    TextSection {
                        value: game_start_msg.to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size: 12.0,
                            color: Color::rgb(1.0, 1.0, 1.0)
                        }
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
            },
            ..Default::default()
        });
    });

}

fn setup_setupgame(
    mut app_state: ResMut<State<AppState>>
) {
    app_state.set(AppState::InGame).unwrap();
}

fn key_press(
    btn: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    if btn.just_pressed(KeyCode::Space) {
        app_state.set(AppState::GameSetup).unwrap();
    }    
}

fn destruct_game(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    entity_query: Query<Entity, Or<(With<TextureAtlasSprite>, With<Sprite>)>>
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }

    state.set(AppState::MainMenu).unwrap();
}

fn destruct_main_menu(
    mut commands: Commands,
    mm_query: Query<Entity, With<UiText>>
) {
    commands.entity(mm_query.single()).despawn_recursive();
}