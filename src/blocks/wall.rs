use crate::{prelude::*, utils::entity_destruct};

pub struct WallPlugin;

impl Plugin for WallPlugin
{
    fn build(&self, _app: &mut App) {

    }
}

pub fn spawn_wall(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_asset: &Res<GameAssets>
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(Wall)
        .insert(StaticEntity)
        .insert(Attackable(TargetPriority::Low))
        .insert(EntityHealth{val: 400.0, func_destruct: entity_destruct})
        .insert(BoxCollider {
            size: Vec2::new(20.0, 20.0)
        });
}