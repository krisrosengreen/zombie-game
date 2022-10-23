use crate::prelude::*;

const TIME_STATE_CHANGE: f32 = 45.0;

pub struct WheatPlugin;

impl Plugin for WheatPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(wheat_behaviour));
    }
}

pub fn wheat_behaviour(
    mut wheat_query: Query<(&mut Wheat, &mut TextureAtlasSprite), With<Wheat>>,
    time: Res<Time>
) {
    for (mut wheat, mut sprite) in wheat_query.iter_mut() {
        if wheat.timer.tick(time.delta()).just_finished() {
            if wheat.state < 2 {
                sprite.index += 1;
                wheat.state += 1;
            }
        }
    }

}

pub fn spawn_wheat(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 17,
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
            val: 1.0,
            func_destruct: wheat_destruct
        })
        .insert(Wheat {
            state: 0,
            timer: Timer::from_seconds(TIME_STATE_CHANGE, true)
        });
}

fn wheat_destruct(
    commands: &mut Commands,
    entity: &Entity,
    _game_assets: &Res<GameAssets>,
    _parent_trans: &Transform
) {
    commands.entity(*entity).despawn();
}