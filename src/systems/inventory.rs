use bevy::sprite::Anchor;

use crate::{prelude::*, utils::destruct_cleanup};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::Inventory)
            .with_system(inventory_init));

        app.add_system_set(SystemSet::on_update(AppState::Inventory)
            .with_system(inventory_handler));

        app.add_system_set(SystemSet::on_exit(AppState::Inventory)
            .with_system(destruct_cleanup::<Inventory>));
    }
}

pub fn inventory_init(
    mut commands: Commands,
    inv_items_query: Query<&InventoryItems, With<Player>>,
    game_assets: Res<GameAssets>,
    inv_texture: Res<InventoryAsset>
) {
    // Spawn all inventory items
    let inv_items = inv_items_query.single();

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
            local: Transform::from_xyz(0.0, 0.0, 30.0),
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
                    local: Transform::from_xyz((i as f32)*xstep + offx, offy, 30.0),
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