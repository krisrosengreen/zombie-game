use crate::prelude::*;

const MAX_CONSTRUCT_DIST: f32 = 50.0;

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(build));
    }
}

fn build(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<&Transform, With<Player>>,
    block: Res<BlockSelection>,
    mut player_inv_q: Query<&mut InventoryItems, With<Player>>,
    btn: Res<Input<MouseButton>>,
    mouseloc: Res<MouseLoc>,
) {

    let player_vec = query.single().translation;
    let mouse_vec: Vec3 = Vec3::new(mouseloc.x, mouseloc.y, 0.0);

    if btn.just_pressed(MouseButton::Right) && player_vec.distance(mouse_vec) < MAX_CONSTRUCT_DIST {
        let mut player_inv = player_inv_q.single_mut();

        if player_inv.has_item(block.block.clone()){
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
            let spawn_trans = Transform::from_translation(spawn_pos);

            match block.block {
                ItemTypes::WallBlock => wall::spawn_wall(&mut commands, spawn_pos, &game_assets),
                ItemTypes::TurretBlock => turret::spawn_turret(&mut commands, spawn_pos, &game_assets),
                ItemTypes::TripMine => tripmine::spawn_tripmine(&mut commands, &game_assets, &Transform::from_translation(spawn_pos)),
                ItemTypes::Fence => fence::spawn_fence(&mut commands, &game_assets, &Transform::from_translation(spawn_pos)),
                ItemTypes::Wheat => wheat::spawn_wheat(&mut commands, &game_assets, &Transform::from_translation(spawn_pos)),
                ItemTypes::WindMill => windmill::spawn_windmill(&mut commands, &game_assets, &Transform::from_translation(spawn_pos)),
                ItemTypes::WoodFence => woodfence::spawn_woodfence(&mut commands, &game_assets, &Transform::from_translation(spawn_pos)),
                ItemTypes::MiningRig => miningrig::spawn_miningrig(&mut commands, spawn_pos, &game_assets),
                _ => println!("Could not match selection type!")
            }

            player_inv.tick_or_remove(block.block.clone());
        }
    }
}