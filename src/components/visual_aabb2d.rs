use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct VisualAabb2d(pub Aabb2d);

impl Default for VisualAabb2d {
    fn default() -> Self {
        VisualAabb2d(Aabb2d::new(Vec2::new(0., 0.), Vec2::splat(32.)))
    }
}