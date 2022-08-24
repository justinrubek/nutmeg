use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::close_on_esc;
use nutmeg_core::camera::systems::camera_follow;
use nutmeg_core::constants::TIMESTEP;
use nutmeg_core::food::systems::{collide_food, player_eat_food};
use nutmeg_core::gui::GuiPlugin;
use nutmeg_core::input::systems::capture_mouse_input;
use nutmeg_core::movement::systems::enforce_speed_limit;
use nutmeg_core::physics::plugin::Rapier2dPhysicsPlugin;
use nutmeg_core::setup;

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(GuiPlugin::default())
        .add_plugin(Rapier2dPhysicsPlugin::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(capture_mouse_input),
        )
        .add_system(camera_follow)
        .add_system(enforce_speed_limit)
        .add_system(player_eat_food)
        .add_system(collide_food)
        .add_system(close_on_esc);

    app
}
