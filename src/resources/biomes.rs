use bevy::prelude::*;   
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Biomes(pub Vec<Biome>);

#[derive(Deserialize, TypePath, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Biome {
    pub name: String,
    pub movement_modifier: f32,
    pub min_height: f32,
    pub min_moisture: f32,
    pub min_heat: f32,
}