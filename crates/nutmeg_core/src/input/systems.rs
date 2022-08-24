use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::camera::data::Camera;
use crate::player::data::LocalPlayer;

/// Determines movement for the player controlled ball.
/// A force will be applied to the ball to move it.
/// The direction of the force is determined by the input.
pub fn capture_mouse_input(
    windows: Res<Windows>,
    mut q: Query<(&LocalPlayer, &mut ExternalForce, &Transform)>,
    camera: Query<&Transform, With<Camera>>,
) {
    let window = windows.get_primary().unwrap();

    let cursor_position = window.cursor_position();
    let (_player, mut ext_force, transform) = q.single_mut();
    let ball_center = transform.translation.truncate();

    if let Some(position) = cursor_position {
        // Determine the world space coords for the cursor
        let norm = Vec3::new(
            position.x - window.width() / 2.,
            position.y - window.height() / 2.,
            0.0,
        );
        let cam_transform = camera.single();
        let world_space = cam_transform.mul_vec3(norm);
        // Change the velocity to be towards the cursor
        // let direction = (position - ball_center).normalize();
        ext_force.force = world_space.truncate() - ball_center;
        ext_force.torque = 1.;
    }
}
