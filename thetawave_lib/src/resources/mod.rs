use crate::components::PlayerComponent;
use amethyst::{
    assets::Handle,
    renderer::{palette::Srgba, SpriteSheet},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod defense;
pub mod drops;
pub mod game_parameters;
pub mod phases;
pub mod store;

pub use self::defense::DefenseResource;
pub use self::drops::{
    DropProbabilities, DropRolls, DropTableType, DropTablesResource, RollProbabilities,
};
pub use self::game_parameters::GameParametersResource;
pub use self::phases::{
    BossType, InvasionFormationPool, InvasionRandomPool, Phase, PhaseManagerResource, PhaseType,
};
pub use self::store::StoreResource;

pub type PlayersResource = HashMap<String, PlayerEntityData>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerEntityData {
    pub player_component: PlayerComponent,
}

#[derive(Clone)]
pub struct SpriteSheetsResource {
    pub spritesheets: HashMap<String, Handle<SpriteSheet>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub image: String,
    pub data: String,
}

pub type SpriteSheetsConfig = HashMap<String, SpriteSheetData>;
pub type SoundsConfig = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DebugLinesConfig {
    pub line_width: f32,
    pub hitbox_color: Srgba,
    pub item_attractor_color: Srgba,
    pub consumable_attractor_color: Srgba,
    pub blast_attractor_color: Srgba,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteRenderData {
    pub initial_index: usize,
    pub spritesheet: String,
}
