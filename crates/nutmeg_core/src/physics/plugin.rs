use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::DEFAULT_PIXELS_PER_METER;

/// A wrapper that configures Rapier2d to use the given world as the physics world alongside common
/// configuration.
pub struct Rapier2dPhysicsPlugin {
    pixels_per_meter: f32,
    debug_draw: bool,
}

impl Default for Rapier2dPhysicsPlugin {
    fn default() -> Self {
        Self {
            pixels_per_meter: DEFAULT_PIXELS_PER_METER,
            debug_draw: true,
        }
    }
}

impl Rapier2dPhysicsPlugin {
    pub fn new(pixels_per_meter: f32, debug_draw: bool) -> Self {
        Self {
            pixels_per_meter,
            debug_draw,
        }
    }
}

impl Plugin for Rapier2dPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            self.pixels_per_meter,
        ));
        if self.debug_draw {
            app.add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}
