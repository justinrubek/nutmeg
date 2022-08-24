use bevy::prelude::*;

#[derive(Component)]
pub struct Food(pub u32);

#[derive(Component)]
pub struct FoodCollector(pub u32);

pub const FOOD_COLLECTORS_GROUP: u32 = 0b0001;
pub const FOOD_GROUP: u32 = 0b0010;
