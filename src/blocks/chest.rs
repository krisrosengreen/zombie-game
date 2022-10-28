use crate::{prelude::*, utils::entity_destruct};

pub struct ChestPlugin;

impl Plugin for ChestPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(chest_init_test));
    }
}

pub fn chest_init_test(
    mut commands: Commands,
    game_assets: Res<GameAssets>
) {
    spawn_chest(&mut commands,
        &game_assets,
        &Transform::from_xyz(0.0, 100.0, 2.5),
        InventoryItems { items: vec![Item{item_type:ItemTypes::Coal, quantity: 20}] });
}

pub fn spawn_chest(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform,
    inv_items: InventoryItems
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ItemTypes::Chest.sprite_index(),
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: parent_trans.clone(),
            ..Default::default()
        })
        .insert(inv_items)
        .insert(InteractableEntity {interact_type: InteractionType::ChestOpen})
        .insert(EntityHealth{val: 500.0, func_destruct: entity_destruct});
}