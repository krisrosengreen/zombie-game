use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin
{
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu)
            .with_system(setup_main_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu)
            .with_system(key_press));     
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    let font_handle: Handle<Font> = asset_server.load("fonts\\Roboto-Regular.ttf");

    commands.spawn_bundle(TextBundle::from_section("Wassup Bitches", TextStyle {
         font: font_handle,
         ..Default::default()
        })).insert_bundle(TransformBundle{
            local: Transform::from_xyz(500.0, 500.0, 0.0),
            ..Default::default()
        });

}

fn key_press(
    btn: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    if btn.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }

    
}