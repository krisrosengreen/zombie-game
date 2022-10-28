use bevy::prelude::*;

pub mod player;

pub use player::MOVESPEED;
pub use player::PLAYER_ACC;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin);
    }
}