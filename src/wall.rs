use bevy::{prelude::*};
use crate::{physics::{StaticEntity, BoxCollider}, entities::EntityHealth, zombie, AppState, GameAssets};

pub struct WallPlugin;

impl Plugin for WallPlugin
{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(setup_walls));
    }
}

#[derive(Component)]
pub struct Wall;

pub fn setup_walls(
    mut _commands: Commands,
    _game_assets: Res<GameAssets>,
) {
    /*
    for i in -10..10 {
        let spawn_pos = Vec3::new(20.0*(i as f32), 100.0, 2.0);

        spawn_wall(&mut commands, spawn_pos, &game_assets);
    }

    for i in -10..10 {
        let spawn_pos = Vec3::new(20.0*(i as f32), -100.0, 2.0);

        spawn_wall(&mut commands, spawn_pos, &game_assets);
    }
    */
}

pub fn spawn_wall(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_asset: &Res<GameAssets>
) {
    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
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
        .insert(zombie::Attackable(zombie::TargetPriority::Low))
        .insert(EntityHealth{val: 400.0, func_destruct: wall_destruct})
        .insert(BoxCollider {
            size: Vec2::new(20.0, 20.0)
        });
}

fn wall_destruct(
    commands: &mut Commands,
    entity: &Entity,
    _game_assets: &Res<GameAssets>,
    _parent_trans: &Transform
) {
    commands.entity(*entity).despawn();
}