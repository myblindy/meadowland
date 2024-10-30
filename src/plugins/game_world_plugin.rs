use crate::bundles::pawn_bundle::*;
use crate::bundles::plant_bundle::*;
use crate::components::{entity_selected::EntitySelected, visual_aabb2d::VisualAabb2d};
use crate::resources::biomes::Biome;
use crate::resources::biomes::Biomes;
use crate::resources::game_resources::GameResource;
use crate::resources::jobs::Jobs;
use crate::systems::ui::*;
use crate::systems::map_generation::*;
use bevy::asset::LoadedFolder;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Resource)]
pub struct GameWorld {
    height: u32,
    width: u32,
    rng: StdRng,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self { height: 0, width: 0, rng: StdRng::from_entropy() }
    }
}


impl GameWorld {
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn cell_size(&self) -> u32 {
        32
    }

    pub fn font_name(&self) -> String {
        "OpenSans-Regular.ttf".to_string()
    }

    pub fn texture_handles_for_biome(&self, biome: &Biome) -> Vec<(Handle<Image>, u32)> {
        todo!()
    }

    pub fn texture_handles_for_all_biomes(&self) -> Vec<Handle<Image>> {
        todo!()
    }

    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    pub fn new(width: u32, height: u32) -> Self {
        GameWorld { width, height, ..default() }
    }
}

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_resource::<GameWorld>();
        app.init_resource::<Jobs>();

        app.add_plugins(JsonAssetPlugin::<GameResource>::new(&["resource.json"]));
        app.add_plugins(JsonAssetPlugin::<Biomes>::new(&["biomes.json"]));
        
        app.add_systems(Startup, start_load_assets);

        app.add_systems(Update, check_assets_loaded.run_if(in_state(GameState::Loading)));
        app.add_systems(Update, run_loading_ui
            .run_if(in_state(GameState::Loading).or_else(in_state(GameState::MapGeneration))));

        app.add_systems(OnEnter(GameState::MapGeneration),start_map_generation);
        app.add_systems(Update, check_map_generation_finished.run_if(in_state(GameState::MapGeneration)));

        app.add_systems(OnEnter(GameState::Main), (create_visual_selection_feedback, generate_world));
        app.add_systems(Update, update_visual_selection_feedback
            .after(TransformSystem::TransformPropagate)
            .run_if(in_state(GameState::Main)));
        app.add_systems(Update, update_plant_harvest_overlay
            .run_if(in_state(GameState::Main)));
        app.add_systems(Update, run_main_ui
            .run_if(in_state(GameState::Main)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MapGeneration,
    Main,
}

#[derive(Resource)]
struct LoadedFolderHandle(Handle<LoadedFolder>);

fn start_load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadedFolderHandle(asset_server.load_folder(".")));
}

fn check_assets_loaded(
    mut app_next_state: ResMut<NextState<GameState>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
){
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _} = event {
            app_next_state.set(GameState::MapGeneration);
        }
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
    selection_query: Query<(&VisualAabb2d, &GlobalTransform), With<EntitySelected>>,
    mut visual_selection_feedback_query: Query<(&mut Path, &mut Visibility, &mut Transform), With<VisualSelectionFeedback>>,
    mut last_visual_selection_feedback_shape_size: Local<Vec2>,
) {
    match selection_query.iter().next() {
        Some((VisualAabb2d(aabb2d), selection_global_transform)) => {
            for (mut path, mut visibility, mut transform) in visual_selection_feedback_query.iter_mut() {
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
            for (_, mut visibility, _) in visual_selection_feedback_query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
