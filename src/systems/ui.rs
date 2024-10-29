use crate::{
    components::{
        entity_selected::*, entity_selected_actions::EntitySelectedActions, nickname::*,
        plant::Plant,
    },
    resources::jobs::*, GameState,
};
use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, *}, EguiContext, EguiContexts};

pub fn run_loading_ui(mut ctx: EguiContexts, state: Res<State<GameState>>) {
    if let Some(ctx) = ctx.try_ctx_mut() {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new(match state.get() {
                    GameState::MapGeneration => "Generating the map...",
                    _ => "Loading...",
                }).font(FontId::proportional(40.0)));
            });
        });
    }
}

pub fn run_main_ui<'a>(
    world: &mut World,
    selected_query: &mut QueryState<&Nickname, With<EntitySelected>>,
    egui_context_query: &mut QueryState<&mut EguiContext, With<PrimaryWindow>>,
    plant_query: &mut QueryState<
        (Entity, &EntitySelectedActions<'static>),
        (With<EntitySelected>, With<Plant>),
    >,
    state: &mut SystemState<Res<Jobs>>,
) {
    // query for the egui context
    let Ok(ctx) = egui_context_query.get_single(world) else {
        return;
    };
    let mut ctx = ctx.clone();
    let ctx = ctx.get_mut();

    egui::TopBottomPanel::bottom("selected_panel").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "Selected: {}",
                match selected_query.iter(world).next() {
                    Some(Nickname(nickname)) => nickname,
                    None => "---",
                }
            ));

            if let Ok((entity, EntitySelectedActions(action_definitions))) =
                plant_query.get_single(world)
            {
                let action_definitions: Vec<_> = action_definitions
                    .into_iter()
                    .map(|w| (w.name.clone(), w.action, w.is_visible))
                    .collect();

                ui.horizontal(|ui| {
                    ui.label("Actions:");

                    for (name, action, is_visible) in action_definitions {
                        if is_visible(world, entity) && ui.button(name).clicked() {
                            action(world, entity);
                        }
                    }
                });
            }
        });
    });

    // query the jobs resource
    let jobs = state
        .get(world)
        .0
        .iter()
        .map(|w| (w.name.clone(), w.job_type.clone()))
        .collect::<Vec<_>>();

    egui::SidePanel::right("Jobs").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Jobs:");

            for (name, job_type) in jobs {
                match job_type {
                    JobType::PlantHarvest(entity) => {
                        ui.label(format!("{name} [{entity}]"));
                    }
                }
            }
        });
    });
}
