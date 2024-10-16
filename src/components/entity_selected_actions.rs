use std::borrow::Cow;

use bevy::prelude::*;

#[derive(Component)]
pub struct EntitySelectedActions<'a>(pub Vec<EntitySelectedAction<'a>>);

pub struct EntitySelectedAction<'a> {
    pub name: Cow<'a, str>,
    pub is_visible: &'a (dyn Fn(&mut World, Entity) -> bool + Send + Sync),
    pub action: &'a (dyn Fn(&mut World, Entity) + Send + Sync),
}