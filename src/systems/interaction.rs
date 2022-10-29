use crate::prelude::*;

pub struct InteractionPlugin;

pub const INTERACTION_DISTANCE: f32 = 60.0;

impl Plugin for InteractionPlugin
{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(chest_open));
    }
}

fn chest_open(
    mut chest_event: EventReader<ChestInteractEvent>,
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    chest_entities: Query<(Entity, &InventoryItems)>
) {
    for cevent in chest_event.iter() {
        let res = chest_entities.iter().find(|p| p.0 == cevent.chest_entity);

        if res.is_some() {
            let (entity_origin, inv_items) = res.unwrap();

            commands
            .spawn()
            .insert(ExternalInventory {entity_origin: entity_origin})
            .insert(inv_items.clone());

            state.set(AppState::ExternalInventory).unwrap();

            break;
        }
    }
}