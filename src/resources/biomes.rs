use bevy::prelude::*;   
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Biomes {
    pub biomes: Vec<Biome>,
}

#[derive(Deserialize, TypePath, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Biome {
    pub name: String,
    pub movement_modifier: f64,
    pub min_height: f64,
    pub min_moisture: f64,
    pub min_heat: f64,
}