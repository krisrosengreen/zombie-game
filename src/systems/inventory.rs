use bevy::sprite::Anchor;

use crate::{prelude::*, utils::destruct_cleanup};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin
{
    fn build(&self, app: &mut App)
    {
        // Player inventory only
        app.add_system_set(SystemSet::on_enter(AppState::Inventory)
            .with_system(player_inventory_init));

        app.add_system_set(SystemSet::on_update(AppState::Inventory)
            .with_system(inventory_handler));

        app.add_system_set(SystemSet::on_exit(AppState::Inventory)
            .with_system(destruct_cleanup::<Inventory>));

        // External inventory
        app.add_system_set(SystemSet::on_enter(AppState::ExternalInventory)
            .with_system(external_inventory_init));

        app.add_system_set(SystemSet::on_update(AppState::ExternalInventory)
            .with_system(inventory_handler));

        app.add_system_set(SystemSet::on_exit(AppState::ExternalInventory)
            .with_system(destruct_cleanup::<Inventory>)
            .with_system(destruct_cleanup::<ExternalInventory>));
    }
}

pub fn player_inventory_init(
    mut commands: Commands,
    inv_items_query: Query<&InventoryItems, With<Player>>,
    game_assets: Res<GameAssets>,
    inv_texture: Res<InventoryAsset>
) {
    // Spawn all inventory items
    inventory_spawn(&mut commands, &inv_items_query.single(), &game_assets, &inv_texture, Vec3::new(0.0, 0.0, 30.0));
}

pub fn external_inventory_init(
    mut commands: Commands,
    player_inv_items_query: Query<&InventoryItems, With<Player>>,
    external_inv_items_query: Query<&InventoryItems, With<ExternalInventory>>,
    game_assets: Res<GameAssets>,
    inv_texture: Res<InventoryAsset>
) {
    // Spawn all inventory items
    inventory_spawn(&mut commands,
        &player_inv_items_query.single(),
        &game_assets,
        &inv_texture,
        Vec3::new(0.0, 55.0, 30.0));
    
    // Spawn all external inventory items
    inventory_spawn(&mut commands,
        &external_inv_items_query.single(),
        &game_assets,
        &inv_texture,
        Vec3::new(0.0, -55.0, 30.0));
}

pub fn inventory_spawn(
    commands: &mut Commands,
    inv_items: &InventoryItems,
    game_assets: &Res<GameAssets>,
    inv_texture: &Res<InventoryAsset>,
    spawn_pos: Vec3
) {
    // Spawn all inventory items
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: inv_texture.texture.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: Anchor::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(Inventory)
        .add_children(|parent| {
            // Start from top left
            let xstep: f32 = 21.0;
            let ystep: f32 = 21.0;

            let offy: f32 = xstep;
            let offx: f32 = -2.0*ystep;

            for i in 0..inv_items.items.len() {
                parent.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: game_assets.texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: inv_items.items[i].item_type as usize,
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(TransformBundle{
                    local: Transform::from_xyz(((i%5) as f32)*xstep + offx, offy - f32::floor((i as f32)/5.0) * ystep, 30.0),
                    ..Default::default()
                })
                .insert(InventoryItem);
            }
        });
}

pub fn inventory_handler(
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
) {
    if keys.clear_just_pressed(KeyCode::I) {
        state.set(AppState::InGame).unwrap();
    }
}