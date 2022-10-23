use bevy::render::camera::RenderTarget;

use crate::prelude::*;

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

pub fn angle_between(a: Vec3, b: Vec3) -> f32{
    (b.y - a.y).atan2(b.x - a.x)
}

pub fn dist_between(a: Vec3, b: Vec3) -> f32 {
    ((b.y - a.y).powf(2.0) + (b.x - a.x).powf(2.0)).sqrt()
}