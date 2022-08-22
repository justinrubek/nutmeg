use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod gui;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(HelloPlugin)
        .add_plugin(gui::GuiPlugin::default())
        .add_system(apply_velocity)
        .run();
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
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

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
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
const BALL_SPEED: f32 = 400.0;
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
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
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        })
        .insert(Velocity(Vec2::new(0.0, BALL_SPEED)));
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
