use bevy::prelude::*;

pub mod inventory;
pub mod environment;
pub mod physics;
pub mod interaction;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugin(inventory::InventoryPlugin)
        .add_plugin(environment::EnvironmentPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(interaction::InteractionPlugin);
    }
}