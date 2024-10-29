use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameResource {
    pub name: String,
    pub weight: f32,
    pub pickup_speed_multiplier: f32,
}
