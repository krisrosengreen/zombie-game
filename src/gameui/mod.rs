use bevy::prelude::*;

pub mod main_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugin(main_menu::MainMenuPlugin);
    }
}