use bevy::render::camera::RenderTarget;

use crate::prelude::{*, interaction::INTERACTION_DISTANCE};

pub fn my_cursor_system(
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

pub fn destruct_cleanup<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
)
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn entity_destruct(
    commands: &mut Commands,
    entity: &Entity,
    _game_assets: &Res<GameAssets>,
    _parent_trans: &Transform
) {
    commands.entity(*entity).despawn_recursive();
}

pub fn angle_between(a: Vec3, b: Vec3) -> f32{
    (b.y - a.y).atan2(b.x - a.x)
}

pub fn dist_between(a: Vec3, b: Vec3) -> f32 {
    ((b.y - a.y).powf(2.0) + (b.x - a.x).powf(2.0)).sqrt()
}

pub fn keyboard_actions(
    mut query_rb: Query<(&mut Rigidbody, &Transform), With<Player>>,
    mut block: ResMut<BlockSelection>,
    mut magazine: Query<&mut Magazine>,
    mut state: ResMut<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    // mouse_loc: Res<MouseLoc>,
    interactables_query: Query<(Entity, &Transform, &InteractableEntity)>,
    time: Res<Time>,

    // Interactions
    mut chest_writer: EventWriter<ChestInteractEvent>
) {
    let (mut rb, player_trans) = query_rb.single_mut();

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

    if input.just_pressed(KeyCode::Key1)
    {
        block.block = ItemTypes::WallBlock;
    }

    if input.just_pressed(KeyCode::Key2)
    {
        block.block = ItemTypes::TurretBlock;
    }

    if input.just_pressed(KeyCode::Key3) {
        block.block = ItemTypes::TripMine;
    }

    if input.just_pressed(KeyCode::Key4) {
        block.block = ItemTypes::Fence;
    }

    if input.just_pressed(KeyCode::Key5) {
        block.block = ItemTypes::Wheat;
    }

    if input.just_pressed(KeyCode::Key6)
    {
        block.block = ItemTypes::WindMill;
    }

    if input.just_pressed(KeyCode::Key7) {
        block.block = ItemTypes::WoodFence;
    }

    if input.just_pressed(KeyCode::Key8) {
        block.block = ItemTypes::MiningRig;
    }

    if input.just_pressed(KeyCode::Key9) {
        block.block = ItemTypes::CraftingTable;
    }

    if input.just_pressed(KeyCode::Key0) {
        block.block = ItemTypes::Chest;
    }

    if input.just_pressed(KeyCode::R)
    {
        let mut magazine = magazine.single_mut();
        magazine.0 = 0;
    }

    if input.clear_just_pressed(KeyCode::I)
    {
        state.set(AppState::Inventory).unwrap();
    }

    if input.just_pressed(KeyCode::E) {
        for (entity, trans, inter_ent) in interactables_query.iter() {
            if (trans.translation - player_trans.translation).length() < INTERACTION_DISTANCE {
                match inter_ent.interact_type {
                    InteractionType::ChestOpen => chest_writer.send(ChestInteractEvent{chest_entity: entity})
                }
            }
        }
    }

    rb.vx = rb.vx.clamp(-MOVESPEED, MOVESPEED);
    rb.vy = rb.vy.clamp(-MOVESPEED, MOVESPEED);

}