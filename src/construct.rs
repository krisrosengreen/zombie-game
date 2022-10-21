use bevy::prelude::*;

use crate::tripmine::spawn_tripmine;
use crate::{MouseLoc, AppState, GameAssets};
use crate::player::Player;
use crate::{wall, turret};

const MAX_CONSTRUCT_DIST: f32 = 50.0;

pub struct ConstructionPlugin;

pub struct BlockSelection
{
    pub block: SelectionTypes
}

impl Plugin for ConstructionPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(build));
    }
}

#[derive(PartialEq, Eq)]
pub enum SelectionTypes
{
    WallBlock,
    TurretBlock,
    TripMine
}

fn build(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<&Transform, With<Player>>,
    block: Res<BlockSelection>,
    btn: Res<Input<MouseButton>>,
    mouseloc: Res<MouseLoc>,
) {

    let player_vec = query.single().translation;
    let mouse_vec: Vec3 = Vec3::new(mouseloc.x, mouseloc.y, 0.0);

    if btn.just_pressed(MouseButton::Right) && player_vec.distance(mouse_vec) < MAX_CONSTRUCT_DIST {

        let x_remain = mouseloc.x%20.0;
        let y_remain = mouseloc.y%20.0;

        let mut x_diff = -x_remain;
        let mut y_diff = -y_remain;

        if x_remain > 10.0 {
            x_diff = 20.0 - x_remain;
        }

        if y_remain > 10.0 {
            y_diff = 20.0 - y_remain;
        }

        let x_pos = mouseloc.x + x_diff;
        let y_pos = mouseloc.y + y_diff;

        let spawn_pos = Vec3::new(x_pos, y_pos, 3.0);

        if block.block == SelectionTypes::WallBlock
        {
            wall::spawn_wall(&mut commands, spawn_pos, &game_assets);
        } else if block.block == SelectionTypes::TurretBlock {
            turret::spawn_turret(&mut commands, spawn_pos, &game_assets);
        } else if block.block == SelectionTypes::TripMine {
            spawn_tripmine(&mut commands, &game_assets, &Transform::from_translation(spawn_pos));
        }
    }
}