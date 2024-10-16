use ::bevy_egui::EguiPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use plugins::game_world_plugin::*;
use quick::WorldInspectorPlugin;

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Meadowland".to_string(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                window_theme: Some(bevy::window::WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    minimize: false,
                    maximize: false,
                    ..default()
                },

                ..default()
            }),
            ..default()
        }))
        .insert_resource(bevy_framepace::FramepaceSettings { limiter: bevy_framepace::Limiter::from_framerate(60.0) })
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins.build()
            .disable::<DefaultHighlightingPlugin>()
            .disable::<SelectionPlugin>())
        .add_plugins(ShapePlugin)
        .insert_resource(GameWorld::new(300, 300))
        .add_plugins(GameWorldPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
