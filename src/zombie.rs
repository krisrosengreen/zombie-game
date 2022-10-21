use bevy::{prelude::*};
use rand::prelude::*;

use crate::{physics::{self, Rigidbody}, angle_between, player::{self}, dist_between, AppState, GameAssets, entities::{self, TempEntity, EntityHealth}};
use std::f32::consts::PI;

pub struct ZombiePlugin;

const ZOMBIE_ACC: f32 = 600.0;
const ZOMB_ANGRY_SPEED: f32 = 180.0;
const ZOMB_IDLE_SPEED: f32 = 30.0;

const ATTACK_DMG: f32 = 10.0;
const START_DIST: f32 = 900.0;
const ATTACK_TIME: f32 = 0.3;
const INIT_TARGET_RAD: f32 = 30.0;

const ENTITY_DIST_REPULSION: f32 = 20.0;
const REPULSION_ACC: f32 = 200.0;

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
pub struct NewTargetTimer(pub Timer);

#[derive(Component)]
pub struct Attackable(pub TargetPriority);

#[derive(Component)]
struct ZombieAttackTimer(Timer);

#[derive(Clone)]
pub enum TargetPriority {
    High = 3,
    Medium = 2,
    Low = 1
}

#[derive(Component)]
pub struct Pathfinder
{
    pub target: Vec3,
    pub target_entity: bool,
    pub target_priority: TargetPriority
}

struct ZombieTimer(Timer);

struct ZombieTimeoutTimer(Timer);

struct ZombieLevelTimer(Timer);

impl Plugin for ZombiePlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .insert_resource(ZombieTimer(Timer::from_seconds(0.3, true)))
        .insert_resource(ZombieTimeoutTimer(Timer::from_seconds(120.0, true)))
        .insert_resource(ZombieLevelTimer(Timer::from_seconds(60.0, true)))
        .add_system_set(SystemSet::on_update(AppState::InGame) 
            .with_system(zombie_ai)
            .with_system(zombie_spawner)
            .with_system(attack_health_entities)
            .with_system(enemy_pathfind)
            .with_system(enemy_entity_pathfind)
            .with_system(mutual_repulsion::<Zombie>)
            .with_system(random_new_target));
    }
}

pub fn zombie_ai(
    mut query: Query<(&Transform, &mut physics::Rigidbody, &Pathfinder), With<Zombie>>,
    time: Res<Time>
) {
    for (zombie, mut rb, pf) in query.iter_mut() {
        let dist = zombie.translation.distance(pf.target);
        let angle = angle_between(zombie.translation, pf.target);

        if dist > 20.0 {
            rb.vx += ZOMBIE_ACC*angle.cos()*time.delta_seconds();
            rb.vy += ZOMBIE_ACC*angle.sin()*time.delta_seconds();
            if pf.target_entity {
                rb.vx = rb.vx.clamp(-ZOMB_ANGRY_SPEED, ZOMB_ANGRY_SPEED);
                rb.vy = rb.vy.clamp(-ZOMB_ANGRY_SPEED, ZOMB_ANGRY_SPEED);
            } else {
                rb.vx = rb.vx.clamp(-ZOMB_IDLE_SPEED, ZOMB_IDLE_SPEED);
                rb.vy = rb.vy.clamp(-ZOMB_IDLE_SPEED, ZOMB_IDLE_SPEED);
            }
        }
    }
}

fn zombie_spawner(
    mut commands: Commands,
    mut spawn_timer: ResMut<ZombieTimer>,
    mut timeout_timer: ResMut<ZombieTimeoutTimer>,
    mut level_timer: ResMut<ZombieLevelTimer>,
    time: Res<Time>,
    game_assets: Res<GameAssets>
) {
    if timeout_timer.0.just_finished() {
        if !level_timer.0.tick(time.delta()).just_finished() {
            if spawn_timer.0.tick(time.delta()).just_finished() {
                let mut rng = rand::thread_rng();
                let angle: f32 = rng.gen::<f32>() * 2.0 * PI;

                let start_pos = Vec3::new(angle.cos() * START_DIST, angle.sin() * START_DIST, 2.0);

                if rng.gen::<f32>() > 0.05 {
                    spawn_zombie(&mut commands, start_pos, &game_assets);
                } else {
                    spawn_chungus_zombie(&mut commands, start_pos, &game_assets);
                }
            }
        } else {
            timeout_timer.0.tick(time.delta());
        }
    } else {
        timeout_timer.0.tick(time.delta());
    }
}

fn attack_health_entities(
    mut health_query: Query<(&Transform, &mut EntityHealth), Without<Zombie>>,
    mut enemy_query: Query<(&Transform, &mut ZombieAttackTimer), With<Zombie>>,
    time: Res<Time>
) {
    for (enm_trans, mut attack_timer) in enemy_query.iter_mut() {
        for (health_trans, mut ent_health) in health_query.iter_mut() {
            if (enm_trans.translation - health_trans.translation).length() < 20.0 {
                if attack_timer.0.tick(time.delta()).just_finished() {
                    ent_health.val = (ent_health.val - ATTACK_DMG).clamp(0.0, 10000.0);
                }
            }
        }
    }
}

fn enemy_pathfind(
    mut query: Query<(&Transform, &mut Pathfinder), With<Pathfinder>>,
    static_query: Query<&Transform, (With<physics::StaticEntity>, Without<Pathfinder>)>,
    player_query: Query<&Transform, (With<player::Player>, Without<physics::StaticEntity>, Without<Pathfinder>)>
) {
    let player = player_query.single();

    for (enm_trans, mut enm_pf) in query.iter_mut() {
        let dist = dist_between(enm_trans.translation, enm_pf.target);

        let r1 = (enm_pf.target.x - enm_trans.translation.x)/dist;
        let r2 = (enm_pf.target.y - enm_trans.translation.y)/dist;

        let b = Vec2::new(r1, r2);
        let a = Vec2::new(enm_trans.translation.x, enm_trans.translation.y);

        let target_vec = enm_pf.target - enm_trans.translation;
        let target_dist = target_vec.length();

        let mut target_obj = false;
        let mut closest_obj_vec = Vec3::ZERO;
        let mut closest_dist = 10000.0;

        for stat_trans in static_query.iter() {
            let x = Vec2::new(stat_trans.translation.x, stat_trans.translation.y);

            let lambda_p = (x.dot(b) - a.dot(b))/b.length_squared();

            let line_dist = (x - (a + lambda_p*b)).length();

            let stat_vec = stat_trans.translation - enm_trans.translation;

            if line_dist < 20.0 && stat_vec.length() < target_dist && stat_vec.dot(target_vec) > 0.0 {
                if stat_vec.length() < closest_dist {
                    closest_dist = stat_vec.length();
                    closest_obj_vec = stat_trans.translation;
                    target_obj = true;
                }
            }
        }

        if (enm_pf.target - closest_obj_vec).length() > 20.0 && (player.translation - enm_pf.target).length() > 5.0 && target_obj {
            enm_pf.target = closest_obj_vec;
        }
    }
}

fn enemy_entity_pathfind(
    mut query: Query<(&Transform, &mut Pathfinder), With<Pathfinder>>,
    static_query: Query<&Transform, (With<physics::StaticEntity>, Without<Pathfinder>)>,
    ent_att: Query<(&Transform, &Attackable), (With<Attackable>, Without<physics::StaticEntity>, Without<Pathfinder>)>
) {
    for (enm_trans, mut enm_pf) in query.iter_mut() {

        let mut found_attackable: bool = false;

        for (ent_att_trans, ent_attackable) in ent_att.iter() {
            let static_vec: Vec<&Transform> = static_query.iter().collect();

            if !is_hindered(&static_vec, &enm_trans, &ent_att_trans) {
                if ent_attackable.0 as u8 > enm_pf.target_priority as u8 {
                    enm_pf.target_entity = true;
                    enm_pf.target = ent_att_trans.translation;
                    enm_pf.target_priority = ent_attackable.0.clone();
                    found_attackable = true;
                }
            }
        }

        if !found_attackable {
            enm_pf.target_entity = false;
            enm_pf.target_priority = TargetPriority::Low;
        }
    }
}

pub fn is_hindered(
    static_vec: &Vec<&Transform>,   //List of positions of static objects
    from_trans: &&Transform,
    to_trans: &&Transform,
) -> bool {
    let dist = dist_between(from_trans.translation, to_trans.translation);

    let r1 = (to_trans.translation.x - from_trans.translation.x)/dist;
    let r2 = (to_trans.translation.y - from_trans.translation.y)/dist;

    let b = Vec2::new(r1, r2);
    let a = Vec2::new(from_trans.translation.x, from_trans.translation.y);

    let target_vec = to_trans.translation - from_trans.translation;
    let target_dist = target_vec.length();

    let is_hindered = false;

    for stat_trans in (*static_vec).iter() {
        let x = Vec2::new(stat_trans.translation.x, stat_trans.translation.y);

        let lambda_p = (x.dot(b) - a.dot(b))/b.length_squared();

        let line_dist = (x - (a + lambda_p*b)).length();

        let stat_vec = stat_trans.translation - from_trans.translation;

        if line_dist < 20.0 && stat_vec.length() < target_dist && stat_vec.dot(target_vec) > 0.0 {
            return true;
        }
    }

    is_hindered
}

fn spawn_zombie(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_assets: &Res<GameAssets>
) {
    let mut rng = rand::thread_rng();

    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(physics::Rigidbody{
            vx: 0.0,
            vy: 0.0,
            friction: true
        })
        .insert(Zombie)
        .insert(Pathfinder{
            target: Vec3::new(rng.gen::<f32>()*INIT_TARGET_RAD,rng.gen::<f32>()*INIT_TARGET_RAD,0.0),
            target_priority: TargetPriority::Low,
            target_entity: false
        })
        .insert(ZombieAttackTimer(Timer::from_seconds(ATTACK_TIME, true)))
        .insert(physics::BoxCollider {
            size: Vec2::new(10.0, 10.0)
        })
        .insert(NewTargetTimer(Timer::from_seconds(5.0, true)))
        .insert(EntityHealth{val: 20.0, func_destruct: zombie_destruct});
}

fn spawn_chungus_zombie(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_assets: &Res<GameAssets>
) {
    let mut rng = rand::thread_rng();

    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform{
                translation: spawn_pos,
                scale: Vec3::ONE * 2.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(physics::Rigidbody{
            vx: 0.0,
            vy: 0.0,
            friction: true
        })
        .insert(Zombie)
        .insert(Pathfinder{
            target: Vec3::new(rng.gen::<f32>()*INIT_TARGET_RAD,rng.gen::<f32>()*INIT_TARGET_RAD,0.0),
            target_priority: TargetPriority::Low,
            target_entity: false
        })
        .insert(ZombieAttackTimer(Timer::from_seconds(ATTACK_TIME, true)))
        .insert(physics::BoxCollider {
            size: Vec2::new(30.0, 30.0)
        })
        .insert(NewTargetTimer(Timer::from_seconds(5.0, true)))
        .insert(EntityHealth{val: 300.0, func_destruct: zombie_destruct});
}

fn zombie_destruct(
    commands: &mut Commands,
    entity: &Entity,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    commands.entity(*entity).despawn();
    
    spawn_dead(commands, parent_trans, game_assets);
}

fn mutual_repulsion<ENTITYTYPE: Component>(
    mut query: Query<(&Transform, &mut Rigidbody), With<ENTITYTYPE>>,
    time: Res<Time>  
) {
    let all_pos: Vec<Vec3> = query.iter().map(|q| q.0.translation).collect();

    for (ent_trans, mut rb) in query.iter_mut() {
        for pos in all_pos.iter() {
            if ent_trans.translation == *pos {
                continue;
            }

            let vec_from = ent_trans.translation - *pos;

            if vec_from.length() <= ENTITY_DIST_REPULSION {
                rb.vx += vec_from.normalize().x*REPULSION_ACC*time.delta_seconds();
                rb.vy += vec_from.normalize().y*REPULSION_ACC*time.delta_seconds();
            }
        }
    }
}

fn random_new_target(
    mut query: Query<(&Transform, &mut NewTargetTimer, &mut Pathfinder)>,
    player_query: Query<&Transform, With<player::Player>>,
    time: Res<Time>
){
    let player = player_query.single();
    let mut rng = rand::thread_rng();

    for (trans, mut timer, mut pf) in query.iter_mut() {
        if (trans.translation - pf.target).length() < 20.0 && !pf.target_entity {
            if timer.0.tick(time.delta()).just_finished() {
                let new_target = player.translation + Vec3::new(rng.gen::<f32>()*INIT_TARGET_RAD, rng.gen::<f32>()*INIT_TARGET_RAD, 0.0);

                pf.target = new_target;
            }
        }
    }
}

fn spawn_dead(
    commands: &mut Commands,
    spawn_trans: &Transform,
    game_assets: &Res<GameAssets>
)
{
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 10,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: spawn_trans.clone(),
            ..Default::default()
        })
        .insert(entities::TempZombieDead::new());
}