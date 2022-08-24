use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::data::LocalPlayer;

use super::data::{Food, FoodCollector};

const FOOD_RADIUS_EXTRA: f32 = 15.;

pub fn player_eat_food(
    mut commands: Commands,
    mut player: Query<(&LocalPlayer, &Transform, &mut FoodCollector)>,
    all_food: Query<&Food>,
    rapier_context: Res<RapierContext>,
) {
    let (_, transform, mut food) = player.single_mut();

    let shape_pos = transform.translation.truncate();
    let shape_rot = 0.;
    let shape_vel = Vec2::new(10., 10.);

    let radius = food.0 as f32 + FOOD_RADIUS_EXTRA;
    let shape = Collider::ball(radius);
    // TODO: Filter only on food predicate
    let filter = QueryFilter::default().exclude_dynamic();

    let collision = rapier_context.cast_shape(shape_pos, shape_rot, shape_vel, &shape, 1.0, filter);
    if let Some((entity, toi)) = collision {
        food.0 += all_food.get(entity).unwrap().0;
        commands.entity(entity).despawn();
    }
}

pub fn collide_food(mut collisions: EventReader<CollisionEvent>, food: Query<&Food>) {
    for collision in collisions.iter() {
        match *collision {
            CollisionEvent::Started(e1, e2, _) => {
                println!("{:?}", e1);
                println!("{:?}", e2);
            }
            _ => {}
        }
    }
}
