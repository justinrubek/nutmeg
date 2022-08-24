use bevy::prelude::*;

use crate::player::data::LocalPlayer;

use super::data::Camera;

#[allow(clippy::type_complexity)]
pub fn camera_follow(
    mut cam: Query<&mut Transform, (With<Camera>, Without<LocalPlayer>)>,
    q: Query<(&Transform, (With<LocalPlayer>, Without<Camera>))>,
) {
    let mut cam = cam.single_mut();
    let (transform, _player) = q.single();

    cam.translation = transform.translation
}
