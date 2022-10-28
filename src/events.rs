use crate::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin
{
    fn build(&self, app: &mut App) {
        app.add_event::<ChestInteractEvent>()
        .add_event::<ChestChangeInventoryEvent>()
        .add_event::<CollisionEvent>();
    }
}

pub struct ChestInteractEvent
{
    pub chest_entity: Entity
}

pub struct ChestChangeInventoryEvent
{
    pub entity: Entity,
    pub items: InventoryItems
}

pub struct CollisionEvent
{
    pub ent_a: Entity,
    pub stat_b: Entity
}