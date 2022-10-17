use bevy::prelude::*;

use crate::MouseLoc;
use crate::{wall, turret};

#[derive(PartialEq, Eq)]
pub enum SelectionTypes
{
    WallBlock,
    TurretBlock
}

pub struct BlockSelection
{
    pub block: SelectionTypes
}

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(build);
    }
}

fn build(
    mut commands: Commands,
    block: Res<BlockSelection>,
    btn: Res<Input<MouseButton>>,
    mouseloc: Res<MouseLoc>,
) {
    if btn.just_pressed(MouseButton::Right) {
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

        let spawn_pos = Vec3::new(x_pos, y_pos, 0.0);

        if block.block == SelectionTypes::WallBlock
        {
            wall::spawn_wall(&mut commands, spawn_pos)
        } else if block.block == SelectionTypes::TurretBlock {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite{
                        color: Color::rgb(0.0,0.0,1.0),
                        custom_size: Some(Vec2::new(20.0,20.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(TransformBundle{
                    local: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..Default::default()
                })
                .insert(turret::Turret)
                .insert(turret::TurretShootTimer(Timer::from_seconds(1.0, true)))
                .insert(turret::TurretBulletTimer(Timer::from_seconds(0.2, true)))
                .insert(turret::TurretCoolTimer(Timer::from_seconds(4.0, true)));
        }
    }
}