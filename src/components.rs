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

#[derive(Component)]
pub struct InventoryItems
{
    pub items: Vec<Item>
}

impl InventoryItems
{
    pub fn get_index(&self, item_type: SelectionTypes) -> usize
    {
        return self
        .items
        .iter()
        .position(|p| p.item_type.eq(&item_type))
        .unwrap_or_else(|| usize::MAX);
    }

    pub fn has_item(&self, item_type: SelectionTypes) -> bool
    {
        self.get_index(item_type) != usize::MAX
    }

    pub fn tick_or_remove(&mut self, item_type: SelectionTypes)
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

#[derive(Clone, PartialEq, Eq)]
pub enum SelectionTypes
{
    WallBlock = 0,
    TurretBlock = 5,
    TripMine = 12,
    Fence = 14,
    Wheat = 17,
    WindMill = 15,
    WoodFence = 20
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    GameSetup,
    InGame,
    Inventory,
    GameDestruct,
    _Paused,
}

#[derive(Component)]
pub struct InventoryItem;

#[derive(Component)]
pub struct UiText;

#[derive(Component)]
pub struct StaticEntity;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Player;

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
pub struct WindMillPlugin;

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