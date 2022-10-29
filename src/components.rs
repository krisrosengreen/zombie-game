use crate::{prelude::*, GameAssets};

#[derive(Component)]
pub struct BoxCollider
{
    pub size: Vec2
}

#[derive(Component)]
pub struct Rigidbody
{
    pub vx: f32,
    pub vy: f32,
    pub friction: bool
}

#[derive(Component)]
pub struct EntityHealth {
    pub val: f32,
    pub func_destruct: fn(&mut Commands, &Entity, &Res<GameAssets>, &Transform),
}

#[derive(Component)]
pub struct Animal
{
    pub stroll_timer: Timer,
    pub stroll_direction: Vec3
}

#[derive(Clone)]
pub enum TargetPriority {
    High = 3,
    Medium = 2,
    Low = 1
}

#[derive(Component)]
pub struct Pathfinder
{
    pub target: Vec3,
    pub target_entity: bool,
    pub target_priority: TargetPriority
}

#[derive(Component)]
pub struct Wheat
{
    pub state: u8,
    pub timer: Timer
}

#[derive(Component)]
pub struct TurretTargeting
{
    pub target: Vec3,
    pub shoot: bool
}

#[derive(Component, Clone)]
pub struct InventoryItems
{
    pub items: Vec<Item>
}

impl InventoryItems
{
    pub fn get_index(&self, item_type: ItemTypes) -> usize
    {
        return self
        .items
        .iter()
        .position(|p| p.item_type.eq(&item_type))
        .unwrap_or_else(|| usize::MAX);
    }

    pub fn has_item(&self, item_type: ItemTypes) -> bool
    {
        self.get_index(item_type) != usize::MAX
    }

    pub fn add_item(&mut self, item: Item) {
        if self.has_item(item.item_type)
        {
            let item_index = self.get_index(item.item_type);
            self.items[item_index].quantity += 1;
        } else {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, item: Item)
    {
        assert!(self.has_item(item.item_type));

        let invitem_index = self.get_index(item.item_type);

        assert!(self.items[invitem_index].quantity >= item.quantity);

        if self.items[invitem_index].quantity == item.quantity {
            self.items.remove(invitem_index);
        } else {
            self.items[invitem_index].quantity -= item.quantity
        }
    }

    pub fn tick_or_remove(&mut self, item_type: ItemTypes)
    {
        let index = self.get_index(item_type);

        if index == usize::MAX {
            return;
        }

        self.items[index].quantity -= 1;

        if self.items[index].quantity == 0 {
            self.items.remove(index);
        }
    }
}

impl Default for InventoryItems
{
    fn default() -> Self {
        Self {
            items: Vec::<Item>::new()
        }
    }
}

#[derive(Component)]
pub struct CollectableItem
{
    pub item: Item
}

#[derive(Component)]
pub struct DropsItem
{
    pub item: Item
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ItemTypes
{
    WallBlock = 0,
    TurretBlock = 5,
    TripMine = 12,
    Fence = 14,
    Wheat = 17,
    WindMill = 15,
    WoodFence = 20,

    // From here on forward, items have not been added.
    Chest = 25,
    LandingPad = 26,
    MiningRig = 27,
    IronIngot = 28,
    Coal = 29,
    CraftingTable = 30,
    Steak = 31
}

#[allow(dead_code)]
impl ItemTypes {
    pub fn sprite_index(&self) -> usize {
        (*self) as usize
    }
}

#[derive(Component)]
pub struct InteractableEntity
{
    pub interact_type: InteractionType
}

#[derive(Component)]
pub struct InventoryItem
{
    pub item: Item
}

#[derive(Component)]
pub struct ExternalInventory
{
    pub entity_origin: Entity
}

#[derive(Component)]
pub struct MiningRig(pub Timer);

#[derive(Component)]
pub struct UiText;

#[derive(Component)]
pub struct StaticEntity;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Chest;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Magazine(pub u8);

#[derive(Component)]
pub struct ReloadTimer(pub Timer);

#[derive(Component)]
pub struct TempZombieDead(pub Timer);

#[derive(Component)]
pub struct TempTurretDestroyed(pub Timer);

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
pub struct NewTargetTimer(pub Timer);

#[derive(Component)]
pub struct Attackable(pub TargetPriority);

#[derive(Component)]
pub struct ZombieAttackTimer(pub Timer);

#[derive(Component)]
pub struct WoodFence;

#[derive(Component)]
pub struct WindMill;

#[derive(Component)]
pub struct WindMillBlade;

#[derive(Clone)]
pub struct WheatPlugin;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct TurretShootTimer(pub Timer);

#[derive(Component)]
pub struct TurretCoolTimer(pub Timer);

#[derive(Component)]
pub struct TurretBulletTimer(pub Timer);

#[derive(Component)]
pub struct TripMine;

#[derive(Component)]
pub struct Explosion(pub Timer);

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Inventory;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct TextScoreboard;