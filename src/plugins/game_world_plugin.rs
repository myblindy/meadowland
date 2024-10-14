use crate::bundles::pawn_bundle::*;
use crate::bundles::plant_bundle::*;
use crate::components::{entity_selected::EntitySelected, visual_aabb2d::VisualAabb2d};
use crate::systems::ui::*;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[derive(Resource, Default)]
pub struct GameWorld {
    height: i32,
    width: i32,
}

impl GameWorld {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn cell_size(&self) -> i32 {
        32
    }

    pub fn font_name(&self) -> String {
        "OpenSans-Regular.ttf".to_string()
    }

    pub fn new(width: i32, height: i32) -> Self {
        GameWorld { width, height }
    }
}

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWorld>();
        app.add_systems(Startup, (create_visual_selection_feedback, generate_world));
        app.add_systems(Update, update_visual_selection_feedback
            .after(TransformSystem::TransformPropagate));
        app.add_systems(Update, create_ui);
    }
}

fn generate_world(
    mut commands: Commands,
    game_world: Res<GameWorld>,
    asset_server: Res<AssetServer>,
) {
    // add a few villagers for testing
    for w in vec![("Sana", 0, 0), ("Mina", 10, 2), ("Eunha", 8, 6)] {
        spawn_pawn(&mut commands, &game_world, &asset_server, w.0, w.1 as f32, w.2 as f32);
    }

    // and a few trees
    for x in 2..5 {
        for y in 2..7 {
            spawn_plant(&mut commands, &game_world, &asset_server, "tree-leafy", x as f32, y as f32);
        }
    }
    for x in -6..4 {
        for y in -10..-4 {
            spawn_plant(&mut commands, &game_world, &asset_server, "tree-pine", x as f32, y as f32);
        }
    }
}

#[derive(Component)]
struct VisualSelectionFeedback;

fn create_visual_selection_feedback(mut commands: Commands) {
    commands.spawn((
        ShapeBundle {
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            ..default()
        },
        Stroke::new(WHITE, 2.0),
        VisualSelectionFeedback,
    ));
}

fn update_visual_selection_feedback(
    selection_query: Query<(&EntitySelected, &VisualAabb2d, &GlobalTransform)>,
    mut visual_selection_feedback_query: Query<(&VisualSelectionFeedback, &mut Path, &mut Visibility, &mut Transform)>,
    mut last_visual_selection_feedback_shape_size: Local<Vec2>,
) {
    match selection_query.iter().next() {
        Some((_, VisualAabb2d(aabb2d), selection_global_transform)) => {
            for (_, mut path, mut visibility, mut transform) in visual_selection_feedback_query.iter_mut() {
                *visibility = Visibility::Visible;
                transform.translation = selection_global_transform.translation();

                let size = aabb2d.max - aabb2d.min;
                if *last_visual_selection_feedback_shape_size != size {
                    *last_visual_selection_feedback_shape_size = size;

                    // build the path from shape
                    let shape = shapes::Rectangle {
                        extents: size / 2.0,
                        ..default()
                    };

                    *path = GeometryBuilder::build_as(&shape);
                }
            }
        }
        None => {
            for (_, _, mut visibility, _) in visual_selection_feedback_query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
