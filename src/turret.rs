use bevy::prelude::*;
use rand::prelude::*;

use crate::{physics::{StaticEntity}, angle_between, zombie::Zombie, weapons::{self, BLLT_RANDOM}, Pathfinder};

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct TurretTargeting
{
    pub target: Vec3,
    pub shoot: bool
}

#[derive(Component)]
pub struct TurretShootTimer(pub Timer);

#[derive(Component)]
pub struct TurretCoolTimer(pub Timer);

#[derive(Component)]
pub struct TurretBulletTimer(pub Timer);

pub struct TurretPlugin;

impl Plugin for TurretPlugin
{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(turret_setup)
        .add_system(turret_targeting);
    }
}

fn turret_setup(
    mut commands: Commands
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(0.0,0.0,1.0),
                custom_size: Some(Vec2::new(20.0,20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_xyz(0.0, 120.0, 0.0),
            ..Default::default()
        })
        .insert(Turret)
        .insert(TurretShootTimer(Timer::from_seconds(1.0, true)))
        .insert(TurretBulletTimer(Timer::from_seconds(0.2, true)))
        .insert(TurretCoolTimer(Timer::from_seconds(4.0, true)));
}

pub fn turret_targeting(
    mut turret_query: Query<(&Transform, &mut TurretShootTimer, &mut TurretCoolTimer, &mut TurretBulletTimer), (With<Turret>, Without<Zombie>)>,
    zombie_query: Query<&Transform, (With<Zombie>, Without<Turret>)>,
    query: Query<(&Transform, &Pathfinder), With<Pathfinder>>,
    static_query: Query<&Transform, (With<StaticEntity>, Without<Pathfinder>)>,
    time: Res<Time>,
    mut commands: Commands
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

    for (turret, mut t_shoot, mut t_cool, mut t_bullet) in turret_query.iter_mut() {
        let mut target_shoot: Vec3 = Vec3::ZERO;
        let mut shoot: bool = false;

        for zombie in zombie_query.iter() {

            if (turret.translation - zombie.translation).length() > 150.0 {
                continue;
            }

            'zombie_loop: for (enm_trans, enm_pf) in query.iter() {
                let dist = (enm_trans.translation-enm_pf.target).length();

                let r1 = (turret.translation.x - enm_trans.translation.x)/dist;
                let r2 = (turret.translation.y - enm_trans.translation.y)/dist;

                let b = Vec2::new(r1, r2);
                let a = Vec2::new(enm_trans.translation.x, enm_trans.translation.y);

                let target_vec = turret.translation - enm_trans.translation;
                let target_dist = target_vec.length();

                for stat_trans in static_query.iter() {
                    let x = Vec2::new(stat_trans.translation.x, stat_trans.translation.y);

                    let lambda_p = (x.dot(b) - a.dot(b))/b.length_squared();

                    let line_dist = (x - (a + lambda_p*b)).length();

                    let stat_vec = stat_trans.translation - enm_trans.translation;

                    if line_dist < 20.0 && stat_vec.length() < target_dist && stat_vec.dot(target_vec) > 0.0 {
                        continue 'zombie_loop;
                    }
                }

                target_shoot = zombie.translation;
                shoot = true;
            }
        }

        if !t_cool.0.just_finished() {
            t_cool.0.tick(time.delta());
        } else {
            if shoot {
                if t_shoot.0.tick(time.delta()).just_finished() {
                    t_cool.0.tick(time.delta());
                }

                if t_bullet.0.tick(time.delta()).just_finished() {

                    // Target zombie and shoot!

                    let angle = angle_between(turret.translation, target_shoot);

                    let mut rng = rand::thread_rng();
                    let rand_angle: f32 = (rng.gen::<f32>() - 0.5) * 2.0*BLLT_RANDOM;

                    weapons::spawn_bullet(&mut commands, turret.translation, angle, rand_angle);

                }
            }
        }
    }

}