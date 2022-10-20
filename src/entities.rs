use bevy::prelude::*;

use crate::{AppState, player::{self}, GameAssets};

#[derive(Component)]
pub struct EntityHealth {
    pub val: f32,
    pub func_destruct: fn(&mut Commands, &Entity, &Res<GameAssets>, &Transform),
}

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(temp_entity_handler)
        .with_system(entity_health)
        .with_system(temp_turret_handler));
    }
}

pub trait TempEntity
{
    fn new() -> Self;

    fn tick(&mut self, time: &Res<Time>);

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

    fn tick(&mut self, time: &Res<Time>)
    {
        self.0.tick(time.delta());
    }

    fn destruct(&self, entity: Entity, commands: &mut Commands)
    {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct TempTurretDestroyed(pub Timer);

impl TempEntity for TempTurretDestroyed
{
    fn new() -> Self
    {
        TempTurretDestroyed(Timer::from_seconds(1.5, false))
    }

    fn tick(&mut self, time: &Res<Time>)
    {
        self.0.tick(time.delta());
    }

    fn destruct(&self, entity: Entity, command: &mut Commands) {
        command.entity(entity).despawn();
    }
}

fn entity_health(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<(Entity, &EntityHealth, &Transform), Without<player::Player>>
) {
    for (entity, health, trans) in query.iter() {
        if health.val <= 0.0 {
            (health.func_destruct)(&mut commands, &entity, &game_assets, &trans);
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
        temp_entity.tick(&time);

        if temp_entity.0.just_finished() {
            temp_entity.destruct(entity, &mut commands);
        }
    }
}

pub fn temp_turret_handler(
    mut query: Query<(Entity, &mut TempTurretDestroyed, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity, mut temp_entity, mut trans) in query.iter_mut() {
        temp_entity.tick(&time);

        trans.translation.y += 40.0*time.delta_seconds();
        trans.scale *= 0.95;

        if temp_entity.0.just_finished() {
            temp_entity.destruct(entity, &mut commands);
        }
    }
}