pub mod gameplay;
pub mod graphics;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicSet;
