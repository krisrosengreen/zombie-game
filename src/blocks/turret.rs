use crate::prelude::{*, weapons::BLLT_RANDOM};

use super::windmill::POWER_RADIUS;

pub struct TurretPlugin;

impl Plugin for TurretPlugin
{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::GameSetup)
            .with_system(turret_setup))
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(turret_targeting));
    }
}

fn turret_setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>
) {
    let spawn_pos = Vec3::new(0.0, 120.0, 3.0);
    spawn_turret(&mut commands, spawn_pos, &game_assets);
}

pub fn spawn_turret(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_assets: &Res<GameAssets>
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 5,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(Turret)
        .insert(TurretShootTimer(Timer::from_seconds(1.5, true)))
        .insert(TurretBulletTimer(Timer::from_seconds(0.5, true)))
        .insert(TurretCoolTimer(Timer::from_seconds(6.0, true)))
        .insert(EntityHealth{val: 300.0, func_destruct: turret_destruct})
        .insert(Attackable(TargetPriority::Medium));
}

pub fn turret_targeting(
    mut turret_query: Query<(&Transform, &mut TurretShootTimer, &mut TurretCoolTimer, &mut TurretBulletTimer), (With<Turret>, Without<Zombie>)>,
    zombie_query: Query<&Transform, (With<Zombie>, Without<Turret>)>,
    static_query: Query<&Transform, (With<StaticEntity>, Without<Pathfinder>)>,
    power_query: Query<&Transform, With<WindMill>>,
    time: Res<Time>,
    mut commands: Commands
) {
    let static_vec: Vec<&Transform> = static_query.iter().collect();

    'outer: for (turret, mut t_shoot, mut t_cool, mut t_bullet) in turret_query.iter_mut() {
        for power_trans in power_query.iter() {
            if (turret.translation - power_trans.translation).length() < POWER_RADIUS {
                let mut target_shoot: Vec3 = Vec3::ZERO;
                let mut shoot: bool = false;

                for zombie in zombie_query.iter() {

                    if (turret.translation - zombie.translation).length() > 150.0 {
                        continue;
                    }
        
                    if !zombie::is_hindered(&static_vec, &turret, &zombie) {
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
                            let rand_angle: f32 = (rng.gen::<f32>() - 0.5) * 7.0 * BLLT_RANDOM;

                            weapons::spawn_bullet(&mut commands, turret.translation, angle, rand_angle);

                        }
                    }
                }

                continue 'outer;
            }
        }
    }
}

fn turret_destruct(
    commands: &mut Commands,
    entity: &Entity,
    game_assets: &Res<GameAssets>,
    parent_trans: &Transform
) {
    commands.entity(*entity).despawn();

    spawn_destroyed(commands, parent_trans.translation, game_assets);
}

fn spawn_destroyed(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_assets: &Res<GameAssets>
)
{
    let mut rng = thread_rng();

    for _ in 0..5
    {
        let rand_vec: Vec3 = Vec3::new(rng.gen::<f32>()*20.0, rng.gen::<f32>()*20.0, rng.gen::<f32>()*20.0);

        (*commands)
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_assets.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: ItemTypes::TurretBlock.sprite_index(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(TransformBundle{
                local: Transform::from_translation(spawn_pos + rand_vec),
                ..Default::default()
            })
            .insert(TempTurretDestroyed::new());
    }
}