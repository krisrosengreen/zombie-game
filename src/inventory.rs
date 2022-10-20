use bevy::{prelude::*};

use crate::AppState;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(inventory_handler));
    }
}

#[derive(Component)]
pub struct PlayerInventory
{

}

pub fn inventory_handler() {

}