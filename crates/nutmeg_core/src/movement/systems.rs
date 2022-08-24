use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::data::SpeedLimit;

pub fn enforce_speed_limit(q: Query<(&mut Velocity, &SpeedLimit)>) {
    for (velocity, speed_limit) in &mut q.iter() {
        let max = Vec2::new(speed_limit.0, speed_limit.0);
        velocity.linvel.clamp(-max, max);
    }
}
