use crate::prelude::*;

const ENTITY_DIST_REPULSION: f32 = 20.0;
const REPULSION_ACC: f32 = 200.0;
const DROPPED_ROTSPEED: f32 = 3.14/2.0;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(temp_entity_handler)
        .with_system(entity_health)
        .with_system(temp_turret_handler)
        .with_system(dropped_behaviour)
        .with_system(drop_items.before(entity_health)));
    }
}

pub trait TempEntity
{
    fn new() -> Self;

    fn tick(&mut self, time: &Res<Time>);

    fn destruct(&self, entity: Entity, command: &mut Commands);
}

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

fn drop_items(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<(&EntityHealth, &Transform, &DropsItem), Without<Player>>
) {
    for (health, trans, drop_items) in query.iter()
    {
        let mut cloned_trans = trans.clone();

        cloned_trans.scale *= 0.5;

        if health.val <= 0.0 {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: game_assets.texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: drop_items.item.item_type as usize,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(TransformBundle{
                    local: cloned_trans,
                    ..Default::default()
                })
                .insert(CollectableItem {
                    item: drop_items.item.clone()
                });
        }
    }
}

fn entity_health(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<(Entity, &EntityHealth, &Transform), Without<Player>>
) {
    for (entity, health, trans) in query.iter() {
        if health.val <= 0.0 {
            (health.func_destruct)(&mut commands, &entity, &game_assets, &trans);
        }
    }
}

fn dropped_behaviour(
    mut query: Query<&mut Transform, With<CollectableItem>>,
    time: Res<Time>
) {
    for mut dropped in query.iter_mut() {
        dropped.rotation = Quat::from_rotation_z(DROPPED_ROTSPEED * (time.seconds_since_startup() as f32));
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

pub fn mutual_repulsion<ENTITYTYPE: Component>(
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