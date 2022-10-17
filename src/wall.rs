use bevy::prelude::*;
use crate::{physics::StaticEntity, player::EntityHealth};

pub struct WallPlugin;

impl Plugin for WallPlugin
{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_walls)
        .add_system(wall_health);
    }
}

#[derive(Component)]
pub struct Wall;

pub fn setup_walls(mut commands: Commands) {
    for i in -10..10 {
        let spawn_pos = Vec3::new(20.0*(i as f32), 100.0, 0.0);

        spawn_wall(&mut commands, spawn_pos);
    }

    for i in -10..10 {
        let spawn_pos = Vec3::new(20.0*(i as f32), -100.0, 0.0);

        spawn_wall(&mut commands, spawn_pos);
    }
}

pub fn spawn_wall(
    commands: &mut Commands,
    spawn_pos: Vec3
) {
    (*commands)
        .spawn_bundle(SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(0.7,0.5,1.0),
                custom_size: Some(Vec2::new(20.0,20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(Wall)
        .insert(StaticEntity)
        .insert(EntityHealth{val: 200.0});
}

pub fn wall_health(
    mut commands: Commands,
    query: Query<(Entity, &EntityHealth), With<Wall>>
) {
    for (wall_ent, health) in query.iter() {
        if health.val <= 0.0 {
            commands.entity(wall_ent).despawn();
        }
    }
}