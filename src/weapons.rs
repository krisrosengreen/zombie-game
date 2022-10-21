use bevy::prelude::*;
use rand::prelude::*;

use crate::{physics::{self, BoxCollider}, dist_between, MouseLoc, player::{self, Player}, angle_between, AppState, entities::EntityHealth, turret::Turret, fence::Fence};

pub const BLLT_SPEED: f32 = 500.0;
pub const BLLT_RANDOM: f32 = 0.1;
pub const MAGAZINE_SIZE: u8 = 30;
pub const BLLT_DMG: f32 = 33.0;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Magazine(pub u8);

#[derive(Component)]
pub struct ReloadTimer(pub Timer);

struct GunTimer(Timer);

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .insert_resource(GunTimer(Timer::from_seconds(0.15, true)))
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(shot_bullets)
            .with_system(shoot)
        );
    }
}

fn shot_bullets(
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    mut commands: Commands,
    mut event_reader: EventReader<physics::CollisionEvent>,
    mut health_query: Query<(&Transform, &mut EntityHealth), (Without<Bullet>, Without<Player>, Without<Turret>, Without<Fence>)>
) {
    let bullet_ents: Vec<Entity> = query.iter().map(|(ent, _trans)| ent).collect();
    
    for coll in event_reader.iter() {
        if bullet_ents.contains(&coll.ent_a) {
            commands.entity(coll.ent_a).despawn();
        }
    }

    // Check if a bullet is near an entity containing the component EntityHealth
    // If so, despawn the bullet and remove health from the entity.
    'outer: for (b_ent, b_trans) in query.iter() {
        for (helt_trans, mut helt_health) in health_query.iter_mut() {
            if (b_trans.translation - helt_trans.translation).length() < 20.0 {
                commands.entity(b_ent).despawn();
                helt_health.val -= BLLT_DMG;
                continue 'outer;
            }
        }
    }

    for (bullet_entity, bullet) in query.iter_mut() {
        if dist_between(Vec3::new(0.0,0.0,0.0), bullet.translation) > 1000.0 {
            commands.entity(bullet_entity).despawn();
            continue;
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut gun_timer: ResMut<GunTimer>,
    mouseloc: Res<MouseLoc>,
    mut query: Query<(&Transform, &mut ReloadTimer, &mut Magazine), With<player::Player>>,
    btn: Res<Input<MouseButton>>,
    time: Res<Time>
) {

    if !gun_timer.0.just_finished() {
        gun_timer.0.tick(time.delta());
    }

    let (player, mut reload_timer, mut magazine) = query.single_mut();
    
    if magazine.0 != 0 {
        //Spawn bullet when left mousebutton is clicked
        if btn.pressed(MouseButton::Left) && gun_timer.0.just_finished(){
            let angle = angle_between(player.translation, Vec3::new(mouseloc.x, mouseloc.y, 0.0));

            let player_pos = player.translation;

            let mut rng = rand::thread_rng();
            let rand_angle: f32 = (rng.gen::<f32>() - 0.5) * BLLT_RANDOM;

            spawn_bullet(&mut commands, player_pos, angle, rand_angle);

            gun_timer.0.tick(time.delta()); // Resume countdown!
            magazine.0 -= 1;
        }
    } else if reload_timer.0.tick(time.delta()).just_finished() {
        magazine.0 = MAGAZINE_SIZE;
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    spawn_at: Vec3,
    angle: f32,
    rand_angle: f32
) {
    (*commands)
        .spawn_bundle(SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(1.0,1.0,0.0),
                custom_size: Some(Vec2::new(5.0,5.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_at),
            ..Default::default()
        })
        .insert(Bullet)
        .insert(physics::Rigidbody{
            vx: (angle + rand_angle).cos()*BLLT_SPEED,
            vy: (angle + rand_angle).sin()*BLLT_SPEED,
            friction: false,
        })
        .insert(BoxCollider{
            size: Vec2::new(5.0, 5.0)
        });
        
}