use bevy::prelude::*;
use bevy::time::FixedTimestep;
use nutmeg_client::gui::GuiPlugin;
use nutmeg_client::{apply_velocity, capture_mouse_input, setup};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(GuiPlugin::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(capture_mouse_input)
                .with_system(apply_velocity.before(capture_mouse_input)),
        )
        .run();
}
