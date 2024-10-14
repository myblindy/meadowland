use super::{entity_selected::*, visual_aabb2d::VisualAabb2d};
use crate::{components::nickname::*, GameWorld};
use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct Pawn;

pub fn spawn_pawn(
    commands: &mut Commands,
    game_world: &Res<GameWorld>,
    asset_server: &Res<AssetServer>,
    name: &str,
    x: f32,
    y: f32,
) {
    commands
        .spawn((
            Name::new("Villager"),
            Nickname(format!("Villager {}", name)),
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.) * game_world.cell_size() as f32,
                    ..default()
                },
                ..default()
            },
            VisualAabb2d(Aabb2d::new(Vec2::new(0., 0.), Vec2::splat(game_world.cell_size() as f32))),
            Pawn,
            PickableBundle::default(),
            On::<Pointer<Click>>::run(select_pawn),
        ))
        .with_children(|parent| {
            // image
            parent.spawn(SpriteBundle {
                texture: asset_server.load("pawns/pawn.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(game_world.cell_size() as f32)),
                    ..default()
                },
                ..default()
            });

            // nameplate
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: name.to_string(),
                        style: TextStyle {
                            font: asset_server.load(format!("fonts/{}", game_world.font_name())),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    game_world.cell_size() as f32 * 0.8,
                    0.,
                )),
                ..default()
            });
        });
}

pub fn select_pawn(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    previous_selected_pawn_query: Query<(Entity, &EntitySelected)>,
    parent_query: Query<&Parent>,
    is_entity_pawn: Query<(Entity, Option<&Pawn>)>,
) {
    for (entity_id, _) in previous_selected_pawn_query.iter() {
        commands.entity(entity_id).remove::<EntitySelected>();
    }

    // we click on the child sprite, but we need to get to the parent Pawn entity
    let mut entity_id = event.target;
    loop {
        if let Ok((_, maybe_pawn)) = is_entity_pawn.get(entity_id) {
            if maybe_pawn.is_some() {
                commands.entity(entity_id).insert(EntitySelected);
                return;
            }
        }

        match parent_query.iter_ancestors(entity_id).next() {
            Some(parent_entity) => {
                entity_id = parent_entity;
            }
            None => return,
        }
    }
}
