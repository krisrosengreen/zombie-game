use bevy::{prelude::*, sprite::Anchor};
use crate::{physics, weapons, zombie, GameAssets, AppState};

pub const MOVESPEED: f32 = 60.0;
pub const PLAYER_ACC: f32 = 600.0;

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component)]
pub struct EntityHealth {
    pub val: f32
}

#[derive(Component)]
pub(crate) struct HealthBar;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(player_setup))
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(player_health));
    }
}

fn player_setup(
    mut commands: Commands,
    game_asset: Res<GameAssets>
) {
    commands
        /*
        .spawn_bundle(SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(0.7,0.7,0.7),
                custom_size: Some(Vec2::new(10.0,10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        */
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            ..Default::default()
        })
        .insert(Player)
        .insert(physics::Rigidbody {
            vx: 0.0,
            vy: 0.0,
            friction: true,
            size: Vec2::new(10.0, 10.0)
        })
        .insert(weapons::ReloadTimer(Timer::from_seconds(2.0, true)))
        .insert(weapons::Magazine(weapons::MAGAZINE_SIZE))
        .insert(zombie::Attackable(zombie::TargetPriority::High))
        .insert(EntityHealth{val: 100.0});

    // SPAWN HEALTBAR
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(0.0,1.0,0.0),
                custom_size: Some(Vec2::new(100.0,10.0)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_xyz(-400.0, 300.0, 0.0),
            ..Default::default()
        })
        .insert(HealthBar);
}

fn player_health(
    mut query: Query<&mut Sprite, With<HealthBar>>,
    player_query: Query<&EntityHealth, With<Player>>
) {

    let mut sprite = query.single_mut();
    let health = player_query.single();
    sprite.custom_size = Some(Vec2 { x: health.val, y: 10.0 });

    sprite.color = Color::rgb(1.0 - health.val/100.0, health.val/100.0, 0.0);
}