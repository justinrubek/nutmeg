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
    if let Some((entity, _toi)) = collision {
        food.0 += all_food.get(entity).unwrap().0;
        commands.entity(entity).despawn();
    }
}

pub fn collide_food(
    mut collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
    all_food: Query<&Food>,
    mut player: Query<(&LocalPlayer, &mut FoodCollector, Entity, &Collider)>,
) {
    let (_, mut food, player_ent, _collider) = player.single_mut();

    for collision in collisions.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision {
            if player_ent == *e1 && all_food.contains(*e2) {
                let new_food_value = all_food.get(*e2).unwrap().0;
                food.0 += new_food_value;
                commands.entity(*e2).despawn();

                // Update the size of the player's collider
                commands
                    .entity(player_ent)
                    .insert(Collider::ball(food.0 as f32));
            }
        }
    }
}
