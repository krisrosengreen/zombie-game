use bevy::sprite::Anchor;

use crate::prelude::*;

pub const COLL_DIST: f32 = 10.0;
pub const MOVESPEED: f32 = 40.0;
pub const PLAYER_ACC: f32 = 600.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .add_system_set(SystemSet::on_enter(AppState::GameSetup)
            .with_system(player_setup))
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(player_health)
            .with_system(collect_items));
    }
}

fn player_setup(
    mut commands: Commands,
    game_asset: Res<GameAssets>
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Rigidbody {
            vx: 0.0,
            vy: 0.0,
            friction: true
        })
        .insert(ReloadTimer(Timer::from_seconds(2.0, true)))
        .insert(Magazine(weapons::MAGAZINE_SIZE))
        .insert(Attackable(TargetPriority::High))
        .insert(BoxCollider {
            size: Vec2::new(10.0, 10.0)
        })
        .insert(InventoryItems{
            items: vec![
                Item{
                    quantity: 30,
                    item_type: ItemTypes::WoodFence
                },
                Item{
                    quantity: 30,
                    item_type: ItemTypes::TurretBlock
                },
                Item{
                    quantity: 30,
                    item_type: ItemTypes::TripMine
                },
                Item{
                    quantity: 30,
                    item_type: ItemTypes::Fence
                },
                Item{
                    quantity: 30,
                    item_type: ItemTypes::WindMill
                },
                Item{
                    quantity: 30,
                    item_type: ItemTypes::Wheat
                },
                Item{
                    quantity: 2,
                    item_type: ItemTypes::MiningRig
                },
                Item{
                    quantity: 2,
                    item_type: ItemTypes::CraftingTable
                },
                Item{
                    quantity: 2,
                    item_type: ItemTypes::Chest
                }
                ]
        })
        .insert(EntityHealth{val: 100.0, func_destruct: player_destruct});

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
            local: Transform::from_xyz(-400.0, 300.0, 10.0),
            ..Default::default()
        })
        .insert(HealthBar);
}

fn player_health(
    mut query: Query<&mut Sprite, With<HealthBar>>,
    player_query: Query<&EntityHealth, With<Player>>,
    mut state: ResMut<State<AppState>>
) {

    let mut sprite = query.single_mut();
    let health = player_query.single();
    sprite.custom_size = Some(Vec2 { x: health.val, y: 10.0 });

    sprite.color = Color::rgb(1.0 - health.val/100.0, health.val/100.0, 0.0);

    if player_query.single().val <= 0.0 {
        state.set(AppState::GameDestruct).unwrap();
    }
}

fn collect_items(
    mut commands: Commands,
    mut player_inv_q: Query<(&Transform, &mut InventoryItems), With<Player>>,
    dropped_items_q: Query<(Entity, &Transform, &CollectableItem), With<CollectableItem>>
) {
    for (player_trans, mut player_invitems) in player_inv_q.iter_mut() {
        for (item_entity, item_trans, item_collable) in dropped_items_q.iter() {
            if (player_trans.translation - item_trans.translation).length() <= COLL_DIST {
                player_invitems.add_item(item_collable.item.clone());
                commands.entity(item_entity).despawn();
            }
        }
    }
}

fn player_destruct(
    _commands: &mut Commands,
    _entity: &Entity,
    _game_assets: &Res<GameAssets>,
    _parent_pos: &Transform
) {

}