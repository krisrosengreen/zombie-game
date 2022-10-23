pub mod animals;
pub mod entities;
pub mod weapons;
pub mod zombie;

pub use weapons::WeaponsPlugin;
pub use animals::AnimalsPlugin;
pub use zombie::ZombiePlugin;
pub use entities::{
    EntitiesPlugin,
    TempEntity,
};

pub use zombie::is_hindered;