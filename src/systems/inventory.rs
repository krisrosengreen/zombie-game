use bevy::sprite::Anchor;

use crate::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::Inventory)
            .with_system(inventory_init));

        app.add_system_set(SystemSet::on_update(AppState::Inventory)
            .with_system(inventory_handler));

        app.add_system_set(SystemSet::on_exit(AppState::Inventory)
            .with_system(inventory_destruct));
    }
}

pub fn inventory_init(
    mut commands: Commands,
    inv_texture: Res<InventoryAsset>
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: inv_texture.texture.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: Anchor::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_xyz(0.0, 0.0, 30.0),
            ..Default::default()
        })
        .insert(Inventory);
}

pub fn inventory_handler(
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
) {
    if keys.clear_just_pressed(KeyCode::I) {
        state.set(AppState::InGame).unwrap();
    }
}

pub fn inventory_destruct(
    mut commands: Commands,
    query: Query<Entity, With<Inventory>>
) {
    commands.entity(query.single()).despawn();
}