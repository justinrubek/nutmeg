use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::camera::data::Camera;
use crate::constants::{BALL_SIZE, BALL_SPEED};
use crate::food::data::FoodCollector;
use crate::player::data::LocalPlayer;

/// Determines movement for the player controlled ball.
/// A force will be applied to the ball to move it.
/// The direction of the force is determined by the input.
pub fn capture_mouse_input(
    windows: Res<Windows>,
    mut q: Query<(&LocalPlayer, &mut Velocity, &Transform, &FoodCollector)>,
    camera: Query<&Transform, With<Camera>>,
) {
    let window = windows.get_primary().unwrap();

    let cursor_position = window.cursor_position();
    let (_player, mut velocity, transform, size) = q.single_mut();
    let ball_center = transform.translation.truncate();

    if let Some(position) = cursor_position {
        // Determine the world space coords for the cursor
        let norm = Vec3::new(
            position.x - window.width() / 2.,
            position.y - window.height() / 2.,
            0.0,
        );
        let cam_transform = camera.single();
        let world_space = cam_transform.mul_vec3(norm).truncate();
        // Determine direction between world_space and position
        let direction = (world_space - ball_center).normalize();

        // Determine the percentage difference the current size is from BALL_SIZE
        let _size_diff = (size.0 as f32 / BALL_SIZE as f32).abs();
        // TODO: Adjust speed based on size_diff

        let new_vel = direction * BALL_SPEED;
        velocity.linvel = new_vel;
    }
}
