use bevy::prelude::*;
use lerp::Lerp;

const DIFFTHRES: f32 = 2.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin
{
    fn build(&self, app: &mut App) {
        app
        .add_system(apply_velocity)
        .add_system(entity_collision);
    }
}

pub struct CollisionEvent
{
    pub ent_a: Entity,
    pub stat_b: Entity
}

#[derive(Component)]
pub struct Rigidbody
{
    pub vx: f32,
    pub vy: f32,
    pub friction: bool,
}

#[derive(Component)]
pub struct StaticEntity;

pub fn apply_velocity(
    mut entity_query: Query<(&mut Transform, &mut Rigidbody), With<Rigidbody>>,
    time: Res<Time>
) {
    let threshold: f32 = 1.0;

    for (mut entity, mut rb) in entity_query.iter_mut() {
        if rb.friction {
            rb.vx = rb.vx.lerp(0.0, time.delta_seconds()*5.0);
            rb.vy = rb.vy.lerp(0.0, time.delta_seconds()*5.0);
        }

        if rb.vx.abs() < threshold {
            rb.vx = 0.0;
        }

        if rb.vy.abs() < threshold {
            rb.vy = 0.0;
        }

        entity.translation.x += rb.vx*time.delta_seconds();
        entity.translation.y += rb.vy*time.delta_seconds();
    }
}

pub fn entity_collision(
    mut entity_query: Query<(Entity, &mut Transform, &mut Rigidbody, &Sprite), With<Rigidbody>>,
    mut event_writer: EventWriter<CollisionEvent>,
    static_query: Query<(Entity, &Transform, &Sprite), (With<StaticEntity>, Without<Rigidbody>)>
) {
    for (
        ent_entity,
        mut ent_trans,
        mut ent_rb,
        ent_sprite
    ) in entity_query.iter_mut() {
        let ent_sprite_size = ent_sprite.custom_size.unwrap();

        for (
            stat_entity,
            stat_trans,
            stat_sprite
        ) in static_query.iter() {
            let stat_sprite_size = stat_sprite.custom_size.unwrap();

            let diff_x = stat_trans.translation.x - ent_trans.translation.x;
            let diff_y = stat_trans.translation.y - ent_trans.translation.y;

            let size_x = ent_sprite_size.x/2.0 + stat_sprite_size.x/2.0 + DIFFTHRES;
            let size_y = ent_sprite_size.y/2.0 + stat_sprite_size.y/2.0 + DIFFTHRES;

            if diff_x.abs() < size_x && stat_trans.translation.y - stat_sprite_size.y/2.0 < ent_trans.translation.y && ent_trans.translation.y < stat_trans.translation.y + stat_sprite_size.y/2.0{
                if diff_x > 0.0 {
                    if ent_rb.vx > 0.0 {
                        ent_rb.vx = 0.0;

                        event_writer.send(CollisionEvent { ent_a: ent_entity, stat_b: stat_entity });

                        ent_trans.translation.x = stat_trans.translation.x - ent_sprite_size.x/2.0 - stat_sprite_size.x/2.0 - DIFFTHRES - 0.1;
                    }
                } else {
                    if ent_rb.vx < 0.0 {
                        ent_rb.vx = 0.0;
                        
                        event_writer.send(CollisionEvent { ent_a: ent_entity, stat_b: stat_entity });
                        
                        ent_trans.translation.x = stat_trans.translation.x + ent_sprite_size.x/2.0 + stat_sprite_size.x/2.0 + DIFFTHRES + 0.1;
                    }
                }
            }

            if diff_y.abs() < size_y && stat_trans.translation.x - stat_sprite_size.x/2.0 < ent_trans.translation.x && ent_trans.translation.x < stat_trans.translation.x + stat_sprite_size.x/2.0{
                if diff_y > 0.0 {
                    if ent_rb.vy > 0.0 {
                        ent_rb.vy = 0.0;
                        
                        event_writer.send(CollisionEvent { ent_a: ent_entity, stat_b: stat_entity });

                        ent_trans.translation.y = stat_trans.translation.y - ent_sprite_size.y/2.0 - stat_sprite_size.y/2.0 - DIFFTHRES + 0.1;
                    }
                } else {
                    if ent_rb.vy < 0.0 {
                        ent_rb.vy = 0.0;
                        
                        event_writer.send(CollisionEvent { ent_a: ent_entity, stat_b: stat_entity });

                        ent_trans.translation.y = stat_trans.translation.y + ent_sprite_size.y/2.0 + stat_sprite_size.y/2.0 + DIFFTHRES + 0.1;
                    }
                }
            }


        }
    }
}