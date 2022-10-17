use bevy::prelude::*;
use rand::prelude::*;

use crate::{physics, Pathfinder, angle_between, player, dist_between};
use std::f32::consts::PI;

pub struct ZombiePlugin;

const ZOMBIE_ACC: f32 = 600.0;
const ZOMB_ANGRY_SPEED: f32 = 180.0;
const ZOMB_IDLE_SPEED: f32 = 30.0;

const ATTACK_DMG: f32 = 10.0;
const START_DIST: f32 = 500.0;
const ATTACK_TIME: f32 = 0.3;
const INIT_TARGET_RAD: f32 = 30.0;

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
struct ZombieAttackTimer(Timer);

struct ZombieTimer(Timer);

impl Plugin for ZombiePlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .insert_resource(ZombieTimer(Timer::from_seconds(2.0, true)))
        .add_system(zombie_ai)
        .add_system(zombie_spawner)
        .add_system(attack_health_entities)
        .add_system(enemy_pathfind)
        .add_system(enemy_player_pathfind);
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
            if pf.angry {
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
    mut timer: ResMut<ZombieTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen::<f32>() * 2.0 * PI;

        let start_pos = Vec3::new(angle.cos() * START_DIST, angle.sin() * START_DIST, 0.0);
        let start_transform = Transform::from_translation(start_pos);

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite{
                    color: Color::rgb(1.0,0.0,0.0),
                    custom_size: Some(Vec2::new(10.0,10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(TransformBundle{
                local: start_transform,
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
                angry: false
            })
            .insert(ZombieAttackTimer(Timer::from_seconds(ATTACK_TIME, true)));
    }
}

fn attack_health_entities(
    mut health_query: Query<(&Transform, &mut player::EntityHealth)>,
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
    // Desired behavior
    // ----------------
    // 
    // Go towards target.
    // 
    // If StaticEntity in the way, either destroy static entity or move around using raycasting
    // 
    // else
    // 
    // be angry and run towards player

    let player = player_query.single();

    for (enm_trans, mut enm_pf) in query.iter_mut() {
        let dist = dist_between(enm_trans.translation, enm_pf.target);

        let r1 = (enm_pf.target.x - enm_trans.translation.x)/dist;
        let r2 = (enm_pf.target.y - enm_trans.translation.y)/dist;

        let b = Vec2::new(r1, r2);
        let a = Vec2::new(enm_trans.translation.x, enm_trans.translation.y);

        let target_vec = enm_pf.target - enm_trans.translation;
        let target_dist = target_vec.length();

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
                }
            }
        }

        if (enm_pf.target - closest_obj_vec).length() > 20.0 && (player.translation - enm_pf.target).length() > 5.0 {
            enm_pf.target = closest_obj_vec;
        }
    }
}

fn enemy_player_pathfind(
    mut query: Query<(&Transform, &mut Pathfinder), With<Pathfinder>>,
    static_query: Query<&Transform, (With<physics::StaticEntity>, Without<Pathfinder>)>,
    player: Query<&Transform, (With<player::Player>, Without<physics::StaticEntity>, Without<Pathfinder>)>
) {
    // Desired behavior
    // ----------------
    // 
    // Go towards target.
    // 
    // If StaticEntity in the way, either destroy static entity or move around using raycasting
    // 
    // else
    // 
    // be angry and run towards player

    let player = player.single();

    for (enm_trans, mut enm_pf) in query.iter_mut() {
        let dist = dist_between(enm_trans.translation, enm_pf.target);

        let r1 = (player.translation.x - enm_trans.translation.x)/dist;
        let r2 = (player.translation.y - enm_trans.translation.y)/dist;

        let b = Vec2::new(r1, r2);
        let a = Vec2::new(enm_trans.translation.x, enm_trans.translation.y);

        let target_vec = player.translation - enm_trans.translation;
        let target_dist = target_vec.length();

        let mut is_hindered = false;

        'inner: for stat_trans in static_query.iter() {
            let x = Vec2::new(stat_trans.translation.x, stat_trans.translation.y);

            let lambda_p = (x.dot(b) - a.dot(b))/b.length_squared();

            let line_dist = (x - (a + lambda_p*b)).length();

            let stat_vec = stat_trans.translation - enm_trans.translation;

            if line_dist < 20.0 && stat_vec.length() < target_dist && stat_vec.dot(target_vec) > 0.0 {
                is_hindered = true;

                break 'inner;
            }
        }

        if !is_hindered {
            enm_pf.target = player.translation;
            enm_pf.angry = true;
        } else {
            enm_pf.angry = false;
        }
    }
}
