use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct VisualAabb2d(pub Aabb2d);