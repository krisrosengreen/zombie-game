use crate::prelude::*;

const ROT_SPEED: f32 = 1.5*3.14;
pub const POWER_RADIUS: f32 = 80.0;

pub struct WindMillPlugin;

impl Plugin for WindMillPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(windmill_behaviour));
    }
}

pub fn windmill_behaviour(
    mut millblade_query: Query<&mut Transform, With<WindMillBlade>>,
    time: Res<Time>
) {
    for mut millblade_trans in millblade_query.iter_mut() {
        let axis = Vec3::new(0.0,0.0,-1.0);

        millblade_trans.rotate_local_axis(axis, ROT_SPEED*time.delta_seconds());
    }
}

pub fn spawn_windmill(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    // Blade of windmill
    let blade_entity = (*commands)
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: game_assets.texture_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: 16,
            custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert_bundle(TransformBundle{
        local: Transform::from_xyz(0.0, 5.0, 6.0),
        ..Default::default()
    })
    .insert(WindMillBlade)
    .id();

    // Windmill structure spawning
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 15,
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
        .insert(StaticEntity)
        .insert(BoxCollider {
            size: Vec2 { x: 20.0, y: 20.0 }
        })
        .insert(EntityHealth {
            val: 200.0,
            func_destruct: windmill_destruct
        })
        .insert(WindMill)
        .add_child(blade_entity);

}

fn windmill_destruct(
    commands: &mut Commands,
    entity: &Entity,
    _game_assets: &Res<GameAssets>,
    _parent_trans: &Transform
) {
    commands.entity(*entity).despawn_recursive();
}