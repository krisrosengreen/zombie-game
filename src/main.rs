mod wall;
mod physics;
mod turret;
mod zombie;
mod player;
mod weapons;
mod construct;

use bevy::{prelude::*, render::camera::RenderTarget}; 
const MOVESPEED: f32 = 60.0;
const PLAYER_ACC: f32 = 600.0;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct TextScoreboard;

#[derive(Component)]
pub struct Pathfinder
{
    pub target: Vec3,
    pub angry: bool
}

struct MouseLoc
{
    x: f32,
    y: f32
}

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .add_plugin(weapons::WeaponsPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(zombie::ZombiePlugin)
    .add_plugin(turret::TurretPlugin)
    .add_plugin(wall::WallPlugin)
    .add_plugin(physics::PhysicsPlugin)
    .add_plugin(construct::ConstructionPlugin)
    .add_system(my_cursor_system)
    .add_system(keyboard_actions)
    .insert_resource(MouseLoc{x: 0.0, y: 0.0})
    .insert_resource(construct::BlockSelection{block: construct::SelectionTypes::WallBlock})
    .add_event::<physics::CollisionEvent>()
    .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default())
    .insert(MainCamera);

}

fn keyboard_actions(
    mut query_rb: Query<&mut physics::Rigidbody, With<player::Player>>,
    mut block: ResMut<construct::BlockSelection>,
    mut magazine: Query<&mut weapons::Magazine>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut rb = query_rb.single_mut();

    if input.pressed(KeyCode::D) {
        rb.vx += PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        rb.vx += -PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::W) {
        rb.vy += PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        rb.vy += -PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::Key1)
    {
        block.block = construct::SelectionTypes::WallBlock;
    }

    if input.pressed(KeyCode::R)
    {
        let mut magazine = magazine.single_mut();
        magazine.0 = 0;
    }

    if input.pressed(KeyCode::Key2)
    {
        block.block = construct::SelectionTypes::TurretBlock;
    }

    rb.vx = rb.vx.clamp(-MOVESPEED, MOVESPEED);
    rb.vy = rb.vy.clamp(-MOVESPEED, MOVESPEED);

}

fn my_cursor_system(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse: ResMut<MouseLoc>
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        mouse.x = world_pos.x;
        mouse.y = world_pos.y;
    }
    
}

pub fn angle_between(a: Vec3, b: Vec3) -> f32{
    (b.y - a.y).atan2(b.x - a.x)
}

fn dist_between(a: Vec3, b: Vec3) -> f32 {
    ((b.y - a.y).powf(2.0) + (b.x - a.x).powf(2.0)).sqrt()
}
