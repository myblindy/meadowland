use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use plugins::game_world_plugin::*;

pub mod plugins;
pub mod components;
pub mod bundles;
pub mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ShapePlugin)

        .insert_resource(GameWorld::new(300, 300))
        .add_plugins(GameWorldPlugin)

        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}