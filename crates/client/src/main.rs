use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::close_on_esc;
use bevy_rapier2d::prelude::*;
use nutmeg_client::gui::GuiPlugin;
use nutmeg_client::{capture_mouse_input, enforce_speed_limit, setup};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(GuiPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(capture_mouse_input),
        )
        // .add_system(camera_follow)
        .add_system(enforce_speed_limit)
        .add_system(close_on_esc)
        .run();
}
