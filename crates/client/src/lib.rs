use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub mod gui;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Person;

pub fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("John".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Jane".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bob".to_string()));
}

pub struct GreetTimer(Timer);

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct LocalPlayer;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // ball
    commands
        .spawn()
        .insert(Ball)
        .insert(LocalPlayer)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        })
        .insert(Velocity(Vec2::new(0.0, 0.0)));
}

const TIME_STEP: f32 = 1.0 / 60.0;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

const MAX_SPEED: f32 = 200.0;

/// Determines movement for the player controlled ball.
pub fn capture_mouse_input(
    windows: Res<Windows>,
    mut q: Query<(&LocalPlayer, &mut Velocity, &Transform)>,
) {
    let window = windows.get_primary().unwrap();

    let cursor_position = window.cursor_position();
    let (_player, mut velocity, transform) = q.single_mut();
    let ball_center = transform.translation.truncate();

    if let Some(position) = cursor_position {
        // Change the velocity to be towards the cursor
        // let direction = (position - ball_center).normalize();
        let direction = (position - ball_center).normalize();
        println!("position {:?}", position);
        println!("direction {:?}", direction);
        println!("ball_center {:?}", ball_center);
        velocity.0 = direction * MAX_SPEED;
    }
}
