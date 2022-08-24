use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::people::data::{Name, Person};

#[derive(Default)]
pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_add_person);
    }
}

#[derive(Default)]
pub struct UiAddPersonState {
    name: String,
}

pub fn ui_add_person(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiAddPersonState>,
    mut commands: Commands,
) {
    egui::Window::new("test").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("name: ");
            ui.text_edit_singleline(&mut ui_state.name);
            if ui.button("add").clicked() {
                commands
                    .spawn()
                    .insert(Person)
                    .insert(Name(ui_state.name.clone()));
            }
        });
    });
}
