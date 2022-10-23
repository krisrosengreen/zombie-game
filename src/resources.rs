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

pub struct InventoryAsset
{
    pub texture: Handle<TextureAtlas>
}

pub struct GunTimer(pub Timer);

pub struct ZombieTimer(pub Timer);

pub struct ZombieTimeoutTimer(pub Timer);

pub struct ZombieLevelTimer(pub Timer);

pub struct BlockSelection
{
    pub block: SelectionTypes
}