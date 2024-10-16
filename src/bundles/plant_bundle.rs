use bevy::{math::bounding::Aabb2d, prelude::*, render::view::NoFrustumCulling};
use bevy_mod_picking::prelude::*;

use crate::{
    components::{
        entity_selected::*, entity_selected_actions::*, nickname::*, plant::*,
        plant_harvest::PlantHarvest, visual_aabb2d::*,
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
    pub entity_selected_actions: EntitySelectedActions<'static>,
}

pub fn spawn_plant(
    commands: &mut Commands,
    game_world: &Res<GameWorld>,
    asset_server: &Res<AssetServer>,
    name: &str,
    x: f32,
    y: f32,
) {
    commands
        .spawn((
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
                entity_selected_actions: EntitySelectedActions(vec![
                    EntitySelectedAction {
                        name: "Harvest".into(),
                        is_visible: &|world, entity_id| {
                            world.entity(entity_id).get::<PlantHarvest>().is_none()
                        },
                        action: &|world, entity_id| {
                            world.entity_mut(entity_id).insert(PlantHarvest);
                        },
                    },
                    EntitySelectedAction {
                        name: "Remove Harvest".into(),
                        is_visible: &|world, entity_id| {
                            world.entity(entity_id).get::<PlantHarvest>().is_some()
                        },
                        action: &|world, entity_id| {
                            world.entity_mut(entity_id).remove::<PlantHarvest>();
                        },
                    },
                ]),
            },
            NoFrustumCulling,
            On::<Pointer<Click>>::run(select_plant),
        ))
        .with_children(|parent| {
            // harvest overlay
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("plants/mark-for-harvest-overlay.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(game_world.cell_size() as f32)),
                        ..default()
                    },
                    ..default()
                },
                NoFrustumCulling,
            ));
        });
}

pub fn select_plant(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    previous_selected_plant_query: Query<Entity, With<EntitySelected>>,
) {
    for entity_id in previous_selected_plant_query.iter() {
        commands.entity(entity_id).remove::<EntitySelected>();
    }

    let entity_id = event.target;
    commands.entity(entity_id).insert(EntitySelected);
}

pub fn update_plant_harvest_overlay(
    plant_query: Query<(Option<&PlantHarvest>, &Children), With<Plant>>,
    mut child_visibility_query: Query<&mut Visibility>,
) {
    for (maybe_plant_harvest, children) in plant_query.iter() {
        if let Some(child) = children.get(0) {
            if let Ok(mut visibility) = child_visibility_query.get_mut(*child) {
                let new_visibility = if maybe_plant_harvest.is_some() {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                
                if *visibility != new_visibility {
                    *visibility = new_visibility;
                }
            }
        }
    }
}
