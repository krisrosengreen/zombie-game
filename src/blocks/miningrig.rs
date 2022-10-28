use crate::{prelude::*, utils::entity_destruct};

use super::windmill::POWER_RADIUS;

pub struct MiningRigPlugin;

const MINED_DROP_RADIUS: f32 = 40.0;
const ITEM_MINE_TIME: f32 = 55.0;

impl Plugin for MiningRigPlugin
{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(miningrig_behaviour));
    }
}

pub fn spawn_miningrig(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_asset: &Res<GameAssets>,
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ItemTypes::MiningRig.sprite_index(),
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(MiningRig(Timer::from_seconds(ITEM_MINE_TIME, true)))
        .insert(StaticEntity)
        .insert(Attackable(TargetPriority::Medium))
        .insert(EntityHealth{val: 1000.0, func_destruct: entity_destruct})
        .insert(BoxCollider {
            size: Vec2::new(20.0, 20.0)
        });
}

fn miningrig_behaviour(
    mut commands: Commands,
    mut rig_query: Query<(&mut MiningRig, &Transform)>,
    power_query: Query<&Transform, With<WindMill>>,
    time: Res<Time>,
    game_asset: Res<GameAssets>
) {
    'outer: for (mut rig, trans) in rig_query.iter_mut() {
        for power_trans in power_query.iter() {
            if (power_trans.translation - trans.translation).length() <= POWER_RADIUS {
                if rig.0.tick(time.delta()).just_finished() {
                    println!("Spawning item!");

                    let mut rng = thread_rng();

                    let rand_radius: f32 = rng.gen::<f32>()*MINED_DROP_RADIUS;
                    let rand_angle: f32 = 2.0 * 3.14 * rng.gen::<f32>();

                    let spawn_pos = Vec3::new(rand_angle.cos()*rand_radius, rand_angle.sin()*rand_radius, 0.0);
                    let spawn_trans = Transform::from_translation(spawn_pos + trans.translation);

                    let drop_types: Vec<ItemTypes> = vec![
                        ItemTypes::IronIngot,
                        ItemTypes::Coal
                    ];
            
                    let drop_item = Item {
                        item_type: drop_types[rng.gen_range(0..=1)],
                        quantity: 1
                    };

                    spawn_dropped(&mut commands,
                        &game_asset,
                        &spawn_trans,
                        drop_item.clone());

                    continue 'outer;
                }
            }
        }
    }
}