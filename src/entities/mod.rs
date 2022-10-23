pub mod animals;
pub mod entities;
pub mod weapons;

pub use weapons::WeaponsPlugin;
pub use entities::{
    EntitiesPlugin,
    TempEntity
};
pub use animals::AnimalsPlugin;