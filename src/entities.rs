use bevy::prelude::*;

use crate::{AppState, player::{EntityHealth, self}};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(temp_entity_handler)
        .with_system(entity_health));
    }
}

pub trait TempEntity
{
    fn new() -> Self;

    fn tick(&self);

    fn destruct(&self, entity: Entity, command: &mut Commands);
}

#[derive(Component)]
pub struct TempZombieDead(pub Timer);

impl TempEntity for TempZombieDead
{
    fn new() -> Self
    {
        TempZombieDead(Timer::from_seconds(5.0, false))
    }

    fn tick(&self)
    {

    }

    fn destruct(&self, entity: Entity, commands: &mut Commands)
    {
        commands.entity(entity).despawn();
    }
}

fn entity_health(
    mut commands: Commands,
    query: Query<(Entity, &EntityHealth), (Without<player::Player>)>
) {
    for (entity, health) in query.iter() {
        if health.val <= 0.0 {
            (health.func_destruct)(&mut commands, &entity);
        }
    }
}

pub fn temp_entity_handler(
    mut query: Query<(Entity, &mut TempZombieDead)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity, mut temp_entity) in query.iter_mut()
    {
        temp_entity.0.tick(time.delta());

        temp_entity.tick();

        if temp_entity.0.just_finished() {
            temp_entity.destruct(entity, &mut commands);
        }
    }
}