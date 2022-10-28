use crate::{prelude::*, utils::entity_destruct};

pub struct WoodFencePlugin;

impl Plugin for WoodFencePlugin
{
    fn build(&self, _app: &mut App) {

    }
}

pub fn spawn_woodfence(
    commands: &mut Commands,
    game_asset: &Res<GameAssets>,
    spawn_trans: &Transform
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ItemTypes::WoodFence.sprite_index(),
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: spawn_trans.clone(),
            ..Default::default()
        })
        .insert(StaticEntity)
        .insert(Attackable(TargetPriority::Low))
        .insert(EntityHealth{val: 400.0, func_destruct: entity_destruct})
        .insert(BoxCollider {
            size: Vec2::new(20.0, 20.0)
        });
}