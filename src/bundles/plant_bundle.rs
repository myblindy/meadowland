use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    components::{
        entity_selected::EntitySelected, nickname::Nickname, plant::Plant,
        visual_aabb2d::VisualAabb2d,
    },
    GameWorld,
};

#[derive(Bundle)]
struct PlantBundle {
    pub plant: Plant,
    pub name: Name,
    pub nickname: Nickname,
    pub visual_aabb2d: VisualAabb2d,
    pub pickable: PickableBundle,
    pub sprite: SpriteBundle,
}

pub fn spawn_plant(
    commands: &mut Commands,
    game_world: &Res<GameWorld>,
    asset_server: &Res<AssetServer>,
    name: &str,
    x: f32,
    y: f32,
) {
    commands.spawn((
        PlantBundle {
            nickname: Nickname(format!("Plant {}", name)),
            sprite: SpriteBundle {
                texture: asset_server.load(format!("plants/{}.png", name)),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(game_world.cell_size() as f32)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.) * game_world.cell_size() as f32,
                    ..default()
                },
                ..default()
            },
            visual_aabb2d: VisualAabb2d(Aabb2d::new(
                Vec2::new(0., 0.),
                Vec2::splat(game_world.cell_size() as f32),
            )),
            pickable: PickableBundle::default(),
            plant: Plant {
                name: name.to_string(),
            },
            name: Name::new(format!("Plant {}", name)),
        },
        On::<Pointer<Click>>::run(select_plant),
    ));
}

pub fn select_plant(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    previous_selected_plant_query: Query<(Entity, &EntitySelected)>,
) {
    for (entity_id, _) in previous_selected_plant_query.iter() {
        commands.entity(entity_id).remove::<EntitySelected>();
    }

    let entity_id = event.target;
    commands.entity(entity_id).insert(EntitySelected);
}
