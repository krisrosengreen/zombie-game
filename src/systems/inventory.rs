use bevy::sprite::Anchor;

use crate::{prelude::*, utils::destruct_cleanup};

pub struct InventoryPlugin;

#[derive(Component, Clone)]
pub struct InvName(pub String);

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
            .with_system(change_inventory)
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
    inventory_spawn(&mut commands,
        &inv_items_query.single(),
        &game_assets,
        &inv_texture,
        Vec3::new(0.0, 0.0, 30.0),
        InvName(String::from("Player")));
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
        Vec3::new(0.0, 55.0, 30.0),
        InvName(String::from("Player")));
    
    // Spawn all external inventory items
    inventory_spawn(&mut commands,
        &external_inv_items_query.single(),
        &game_assets,
        &inv_texture,
        Vec3::new(0.0, -55.0, 30.0),
        InvName(String::from("External")));
}

pub fn inventory_spawn(
    commands: &mut Commands,
    inv_items: &InventoryItems,
    game_assets: &Res<GameAssets>,
    inv_texture: &Res<InventoryAsset>,
    spawn_pos: Vec3,
    inv_name: InvName
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
        .insert(inv_name.clone())
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
                .insert(InventoryItem{item: inv_items.items[i].clone()});
            }
        });
}

pub fn change_inventory(
    mouse: Res<Input<MouseButton>>,
    mouse_loc: Res<MouseLoc>,
    inv_item_query: Query<(&Parent, &GlobalTransform, &InventoryItem), (With<InventoryItem>, Without<Player>)>,
    inventory_frame_query: Query<(Entity, &InvName), With<Inventory>>,
    mut all_inventory_entities: Query<(Entity, &mut InventoryItems), (Without<Inventory>, Without<Player>, Without<ExternalInventory>)>,
    mut player_inv: Query<(Entity, &mut InventoryItems), With<Player>>,
    mut state: ResMut<State<AppState>>,
    external_inv: Query<&ExternalInventory, (With<ExternalInventory>, Without<Player>)>
) {
    if !player_inv.is_empty() && !external_inv.is_empty() && !inventory_frame_query.is_empty() {
        let player_invframe_entity = inventory_frame_query.iter().find(|p| p.1.0 == "Player").unwrap().0;
        let external_invframe_entity = inventory_frame_query.iter().find(|p| p.1.0 == "External").unwrap().0;
        
        let mut player_invitems = player_inv.single_mut().1;
        let external_inventory = external_inv.single();

        let external_origin_entity = external_inventory.entity_origin;
        let mut external_origin_invitems = all_inventory_entities.iter_mut()
            .find(|p| p.0.eq(&external_origin_entity))
            .unwrap().1;

        if mouse.just_pressed(MouseButton::Right) {
            for (parent_entity, glob_trans, invitem) in inv_item_query.iter() {

                let mut length = glob_trans.translation() - mouse_loc.get_vec3();
                length.z = 0.0;

                if length.length() < 10.0 {
                    if parent_entity.get().eq(&player_invframe_entity)  {
                            println!("Removing from player inventory!");

                            external_origin_invitems.add_item(invitem.item.clone());
                            player_invitems.remove_item(invitem.item.clone());
                    } else if parent_entity.get().eq(&external_invframe_entity) {
                            println!("Removing from external inventory!");

                            external_origin_invitems.remove_item(invitem.item.clone());
                            player_invitems.add_item(invitem.item.clone());
                    }

                    break;
                }
            }
        }
    } else {
        println!("It is empty!");
    }
}

pub fn inventory_handler(
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
) {
    if keys.clear_just_pressed(KeyCode::I) {
        state.set(AppState::InGame).unwrap();
    }
}