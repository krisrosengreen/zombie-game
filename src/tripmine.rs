use bevy::prelude::*;

const TRIGGER_DIST: f32 = 20.0;
const BLAST_RADIUS: f32 = 100.0;
const EXPLOSION_DMG_PER_FRAME: f32 = 200.0;
// const EXPLOSIVE_ACC: f32 = 400.0;
const EXPLOSION_TIME: f32 = 0.5;

pub struct TripMinePlugin;

use crate::{zombie::{Zombie}, AppState, entities::EntityHealth, GameAssets, physics::{StaticEntity}};

impl Plugin for TripMinePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(tripmine_detonator)
        .with_system(explosion_behaviour));
    }
}

#[derive(Component)]
pub struct TripMine;

#[derive(Component)]
pub struct Explosion(pub Timer);

fn tripmine_detonator(
    mut commands: Commands,
    tripmine_query: Query<(Entity, &Transform), With<TripMine>>,
    zombie_query: Query<&Transform, With<Zombie>>,
    game_assets: Res<GameAssets>
) {
    'outer: for (tm_entity, tm_trans) in tripmine_query.iter() {
        for zomb_trans in zombie_query.iter() {
            if (tm_trans.translation - zomb_trans.translation).length() < TRIGGER_DIST {
                tripwire_destruct(&mut commands, &tm_entity, &game_assets, &tm_trans);
                continue 'outer;
            }
        }
    }
}

pub fn spawn_tripmine(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 12,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: parent_trans.clone(),
            ..Default::default()
        })
        .insert(StaticEntity)
        .insert(TripMine);
}

fn tripwire_destruct(
    commands: &mut Commands,
    entity: &Entity,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    commands.entity(*entity).despawn();
    spawn_explosion(commands, game_assets, parent_trans)
}

fn spawn_explosion(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    let expl_trans = parent_trans.clone().with_scale(Vec3::ONE*4.0);

    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 13,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: expl_trans,
            ..Default::default()
        })
        .insert(Explosion(Timer::from_seconds(EXPLOSION_TIME, false)));
}

fn explosion_behaviour(
    mut commands: Commands,
    mut expl_query: Query<(Entity, &mut Transform, &mut Explosion), With<Explosion>>,
    mut zombie_query: Query<(&Transform, &mut EntityHealth), (With<EntityHealth>, Without<Explosion>)>,
    time: Res<Time>
) {
    for (expl_entity, mut expl_trans, mut expl_expl) in expl_query.iter_mut() {
        for (zombie_trans, mut zombie_health) in zombie_query.iter_mut() {
            let expl_to_zomb: Vec3 = zombie_trans.translation - expl_trans.translation;            
            if expl_to_zomb.length() <= BLAST_RADIUS  {
                zombie_health.val -= EXPLOSION_DMG_PER_FRAME*time.delta_seconds();
            }
        }

        expl_trans.scale *= 0.85;
        if expl_expl.0.tick(time.delta()).just_finished() {
            commands.entity(expl_entity).despawn();
        }
    }
}