use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Person;

pub struct GreetTimer(pub Timer);
