mod player;
mod systems;
mod gameui;
mod entities;
mod blocks;
mod components;
mod resources;
mod utils;

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
    pub use crate::utils::angle_between;
    pub use crate::utils::dist_between;
    pub use crate::utils::my_cursor_system;
}

use prelude::*;

pub const MOVESPEED: f32 = 40.0;
pub const PLAYER_ACC: f32 = 600.0;

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .insert_resource(MouseLoc{x: 0.0, y: 0.0})
    .insert_resource(BlockSelection{block: SelectionTypes::WallBlock})
    .add_event::<CollisionEvent>()
    .add_state(AppState::MainMenu)
    .add_plugin(WeaponsPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(ZombiePlugin)
    .add_plugin(TurretPlugin)
    .add_plugin(WallPlugin)
    .add_plugin(PhysicsPlugin)
    .add_plugin(ConstructionPlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(InventoryPlugin)
    .add_plugin(EntitiesPlugin)
    .add_plugin(EnvironmentPlugin)
    .add_plugin(TripMinePlugin)
    .add_plugin(FencePlugin)
    .add_plugin(WheatPlugin)
    .add_plugin(WindMillPlugin)
    .add_plugin(AnimalsPlugin)
    .add_system_set(SystemSet::on_update(AppState::InGame) 
        .with_system(my_cursor_system)
        .with_system(keyboard_actions)
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
    Vec2::new(20.0,20.0), 4, 6);

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

fn keyboard_actions(
    mut query_rb: Query<&mut Rigidbody, With<Player>>,
    mut block: ResMut<BlockSelection>,
    mut magazine: Query<&mut Magazine>,
    mut state: ResMut<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut rb = query_rb.single_mut();

    if input.pressed(KeyCode::D) {
        rb.vx += PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        rb.vx += -PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::W) {
        rb.vy += PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        rb.vy += -PLAYER_ACC*time.delta_seconds();
    }

    if input.just_pressed(KeyCode::Key1)
    {
        block.block = SelectionTypes::WallBlock;
    }

    if input.just_pressed(KeyCode::Key2)
    {
        block.block = SelectionTypes::TurretBlock;
    }

    if input.just_pressed(KeyCode::Key3) {
        block.block = SelectionTypes::TripMine;
    }

    if input.just_pressed(KeyCode::Key4) {
        block.block = SelectionTypes::Fence;
    }

    if input.just_pressed(KeyCode::Key5) {
        block.block = SelectionTypes::Wheat;
    }

    if input.just_pressed(KeyCode::Key6)
    {
        block.block = SelectionTypes::WindMill;
    }

    if input.just_pressed(KeyCode::Key7) {
        block.block = SelectionTypes::WoodFence;
    }

    if input.just_pressed(KeyCode::R)
    {
        let mut magazine = magazine.single_mut();
        magazine.0 = 0;
    }

    if input.clear_just_pressed(KeyCode::I)
    {
        state.set(AppState::Inventory).unwrap();
    }

    rb.vx = rb.vx.clamp(-MOVESPEED, MOVESPEED);
    rb.vy = rb.vy.clamp(-MOVESPEED, MOVESPEED);

}