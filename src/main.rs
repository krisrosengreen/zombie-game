mod player;
mod systems;
mod gameui;
mod entities;
mod blocks;
mod components;
mod resources;
mod utils;
mod events;

mod prelude {
    pub use bevy::prelude::*;
    pub use rand::*;
    pub use crate::player::*;
    pub use crate::gameui::*;
    pub use crate::systems::*;
    pub use crate::entities::*;
    pub use crate::blocks::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::events::*;
    pub use crate::utils::angle_between;
    pub use crate::utils::dist_between;
    pub use crate::utils::my_cursor_system;
}

use prelude::*;

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .insert_resource(MouseLoc{x: 0.0, y: 0.0})
    .insert_resource(BlockSelection{block: ItemTypes::WallBlock})
    .add_state(AppState::MainMenu)
    .add_plugin(PlayerPlugin)
    .add_plugin(EntitiesPlugin)
    .add_plugin(GameUiPlugin)
    .add_plugin(SystemsPlugin)
    .add_plugin(BlocksPlugin)
    .add_plugin(EventsPlugin)
    .add_system_set(SystemSet::on_update(AppState::InGame) 
        .with_system(my_cursor_system)
        .with_system(utils::keyboard_actions)
    )
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(Camera2dBundle::default())
    .insert(MainCamera);

    // Get the texture sheet
    let texture_handle = asset_server.load("Sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
    Vec2::new(20.0,20.0), 4, 8);

    let texture_atlas_handle = (texture_atlases).add(texture_atlas);
    
    commands.insert_resource(GameAssets{
        texture_atlas: texture_atlas_handle.clone()
    });

    // Get the texture of the inventory
    let texture_handle_inventory = asset_server.load("inventory.png");
    let texture_atlas_inventory = TextureAtlas::from_grid(texture_handle_inventory,
        Vec2::new(108.0, 66.0), 1, 1);

    let inventory_handle = texture_atlases.add(texture_atlas_inventory);

    commands.insert_resource(InventoryAsset {
        texture: inventory_handle
    });
}