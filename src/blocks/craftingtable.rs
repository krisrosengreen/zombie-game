use crate::{prelude::*, utils::entity_destruct};

pub struct CraftingTablePlugin;

impl Plugin for CraftingTablePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame));
    }
}

pub fn spawn_craftingtable(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ItemTypes::CraftingTable.sprite_index(),
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: parent_trans.clone(),
            ..Default::default()
        })
        .insert(Attackable(TargetPriority::Low))
        .insert(EntityHealth {
            val: 50.0,
            func_destruct: entity_destruct 
        });
}