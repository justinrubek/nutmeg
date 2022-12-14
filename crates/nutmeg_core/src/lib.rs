use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use constants::{BALL_SIZE, BALL_STARTING_POSITION, FOOD_SIZE};
use food::data::{Food, FoodCollector};
use movement::data::SpeedLimit;
use rand::prelude::*;

pub mod camera;
pub mod constants;
pub mod food;
pub mod gui;
pub mod input;
pub mod movement;
pub mod people;
pub mod physics;
pub mod player;

use camera::data::Camera;
use player::data::{Ball, LocalPlayer};

pub fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Camera);

    // ball
    commands
        .spawn()
        .insert(Ball)
        .insert(LocalPlayer)
        .insert(FoodCollector(BALL_SIZE as u32))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::ball(BALL_SIZE))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .insert(Sensor)
        .insert_bundle(TransformBundle::from(Transform::from_translation(
            BALL_STARTING_POSITION,
        )))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.0,
        })
        .insert(SpeedLimit(0.1))
        .insert(ColliderMassProperties::Density(2.0));

    // Food
    let mut random = rand::thread_rng();
    for _ in 0..100 {
        let distance = 2000.0;
        let x = random.gen_range(-distance..distance);
        let y = random.gen_range(-distance..distance);
        let position = Vec3::new(x, y, 0.0);

        commands
            .spawn()
            .insert(Food(5))
            .insert(Collider::ball(FOOD_SIZE))
            .insert_bundle(TransformBundle::from(Transform::from_translation(position)));
    }
}
