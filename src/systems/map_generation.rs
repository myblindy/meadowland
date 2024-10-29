use bevy::tasks::futures_lite::future;
use bevy::{prelude::*, tasks::*};
use crate::plugins::game_world_plugin::GameState;

#[derive(Resource)]
pub struct MapGenerationTask(Task<()>);

pub fn start_map_generation(
    mut commands: Commands,
) {
    let task_pool = AsyncComputeTaskPool::get();
    let task = task_pool.spawn(async move {
        // generate the map
    });

    commands.insert_resource(MapGenerationTask(task));
}

pub fn check_map_generation_finished(
    mut commands: Commands,
    mut task: ResMut<MapGenerationTask>,
    mut app_next_state: ResMut<NextState<GameState>>,
) {
    let status = block_on(future::poll_once(&mut task.0));
    if status.is_some() {
        commands.remove_resource::<MapGenerationTask>();
        app_next_state.set(GameState::Main);
    }
}