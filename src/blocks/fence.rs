use crate::{prelude::*, utils::entity_destruct};

const FENCE_SPEED: f32 = 3.0;

pub struct FencePlugin;

impl Plugin for FencePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(fence_behaviour));
    }
}

pub fn spawn_fence(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 14,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: parent_trans.clone(),
            ..Default::default()
        })
        .insert(EntityHealth{val: 500.0, func_destruct: entity_destruct})
        .insert(Fence);
}

fn fence_behaviour(
    mut entity_query: Query<(&Transform, &mut Rigidbody), (Without<Fence>, Without<Bullet>)>,
    fence_query: Query<&Transform, With<Fence>>
) {
    for (ent_trans, mut ent_rb) in entity_query.iter_mut() {
        for fence_trans in fence_query.iter() {
            if (ent_trans.translation - fence_trans.translation).length() < 20.0 {
                ent_rb.vx = ent_rb.vx.clamp(-FENCE_SPEED, FENCE_SPEED);
                ent_rb.vy = ent_rb.vy.clamp(-FENCE_SPEED, FENCE_SPEED);
            }
        }
    }
}
