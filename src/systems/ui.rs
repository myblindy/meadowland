use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{entity_selected::*, nickname::*};

pub fn create_ui(
    mut ctx: EguiContexts,
    selected_query: Query<&Nickname, With<EntitySelected>>,
) {
    let Some(ctx) = ctx.try_ctx_mut() else { return };

    egui::TopBottomPanel::bottom("selected_panel").show(ctx, |ui| {
        ui.label(format!("Selected: {}", match selected_query.iter().next() {
            Some(Nickname(nickname)) => nickname,
            None => "---",
        }));
    });
}