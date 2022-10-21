mod wall;
mod physics;
mod turret;
mod zombie;
mod player;
mod weapons;
mod construct;
mod main_menu;
mod inventory;
mod entities;
mod environment;
mod tripmine;
mod fence;
mod wheat;

use bevy::{prelude::*, render::camera::RenderTarget}; 

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct TextScoreboard;

struct MouseLoc
{
    x: f32,
    y: f32
}

pub struct GameAssets
{
    pub texture_atlas: Handle<TextureAtlas>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    _Paused,
}

fn main() {
    App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .insert_resource(MouseLoc{x: 0.0, y: 0.0})
    .insert_resource(construct::BlockSelection{block: construct::SelectionTypes::WallBlock})
    .add_event::<physics::CollisionEvent>()
    .add_state(AppState::MainMenu)
    .add_plugin(weapons::WeaponsPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(zombie::ZombiePlugin)
    .add_plugin(turret::TurretPlugin)
    .add_plugin(wall::WallPlugin)
    .add_plugin(physics::PhysicsPlugin)
    .add_plugin(construct::ConstructionPlugin)
    .add_plugin(main_menu::MainMenuPlugin)
    .add_plugin(inventory::InventoryPlugin)
    .add_plugin(entities::EntitiesPlugin)
    .add_plugin(environment::EnvironmentPlugin)
    .add_plugin(tripmine::TripMinePlugin)
    .add_plugin(fence::FencePlugin)
    .add_plugin(wheat::WheatPlugin)
    .add_system_set(SystemSet::on_update(AppState::InGame) 
        .with_system(my_cursor_system)
        .with_system(keyboard_actions)
    )
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(Camera2dBundle::default())
    .insert(MainCamera);

    // Get the texture sheet
    let texture_handle = asset_server.load("Sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
    Vec2::new(20.0,20.0), 4, 6);

    let texture_atlas_handle = (texture_atlases).add(texture_atlas);
    
    commands.insert_resource(GameAssets{
        texture_atlas: texture_atlas_handle.clone()
    });

    // Get the texture of the inventory
    
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
        rb.vx += player::PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        rb.vx += -player::PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::W) {
        rb.vy += player::PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        rb.vy += -player::PLAYER_ACC*time.delta_seconds();
    }

    if input.pressed(KeyCode::Key1)
    {
        block.block = construct::SelectionTypes::WallBlock;
    }

    if input.pressed(KeyCode::Key2)
    {
        block.block = construct::SelectionTypes::TurretBlock;
    }

    if input.pressed(KeyCode::Key3) {
        block.block = construct::SelectionTypes::TripMine;
    }

    if input.pressed(KeyCode::Key4) {
        block.block = construct::SelectionTypes::Fence;
    }

    if input.pressed(KeyCode::Key5) {
        block.block = construct::SelectionTypes::Wheat;
    }

    if input.pressed(KeyCode::R)
    {
        let mut magazine = magazine.single_mut();
        magazine.0 = 0;
    }

    rb.vx = rb.vx.clamp(-player::MOVESPEED, player::MOVESPEED);
    rb.vy = rb.vy.clamp(-player::MOVESPEED, player::MOVESPEED);

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
