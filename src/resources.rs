use crate::prelude::*;

pub struct MouseLoc
{
    pub x: f32,
    pub y: f32
}

pub struct GameAssets
{
    pub texture_atlas: Handle<TextureAtlas>
}

pub struct CollisionEvent
{
    pub ent_a: Entity,
    pub stat_b: Entity
}

pub struct BlockSelection
{
    pub block: ItemTypes
}

pub struct InventoryAsset
{
    pub texture: Handle<TextureAtlas>
}

#[derive(Clone)]
pub struct Item
{
    pub item_type: ItemTypes,
    pub quantity: i8
}

pub struct GunTimer(pub Timer);

pub struct ZombieTimer(pub Timer);

pub struct ZombieTimeoutTimer(pub Timer);

pub struct ZombieLevelTimer(pub Timer);