use bevy::prelude::*;

pub mod animals;
pub mod entities;
pub mod weapons;
pub mod zombie;

pub use entities::{
    TempEntity,
};

pub use zombie::is_hindered;
pub use entities::spawn_dropped;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugin(weapons::WeaponsPlugin)
        .add_plugin(animals::AnimalsPlugin)
        .add_plugin(zombie::ZombiePlugin)
        .add_plugin(entities::EntitiesPlugin);
    }
}