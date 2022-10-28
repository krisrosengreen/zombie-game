use bevy::prelude::*;

pub mod construct;
pub mod fence;
pub mod tripmine;
pub mod turret;
pub mod wheat;
pub mod windmill;
pub mod woodfence;
pub mod wall;
pub mod miningrig;
pub mod craftingtable;
pub mod chest;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugin(construct::ConstructionPlugin)
        .add_plugin(fence::FencePlugin)
        .add_plugin(tripmine::TripMinePlugin)
        .add_plugin(turret::TurretPlugin)
        .add_plugin(wheat::WheatPlugin)
        .add_plugin(windmill::WindMillPlugin)
        .add_plugin(woodfence::WoodFencePlugin)
        .add_plugin(wall::WallPlugin)
        .add_plugin(miningrig::MiningRigPlugin)
        .add_plugin(craftingtable::CraftingTablePlugin)
        .add_plugin(chest::ChestPlugin);
    }
}