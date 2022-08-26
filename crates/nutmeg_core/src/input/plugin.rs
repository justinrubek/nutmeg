use bevy::prelude::*;

/// A wrapper that configures Rapier2d to use the given world as the physics world alongside common
/// configuration.
#[derive(Default)]
pub struct InputPlugin;

impl InputPlugin {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {}
}
