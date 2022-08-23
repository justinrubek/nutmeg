use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
const BALL_SIZE: f32 = 30.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct Camera;

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
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_SIZE))
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
        .insert(SpeedLimit(50.0))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(ExternalForce {
            force: Vec2::new(2.0, 0.0),
            torque: 0.0,
        });
    /*
    .insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(BALL_COLOR)),
        transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
        ..default()
    });
    */
}

/// Determines movement for the player controlled ball.
pub fn capture_mouse_input(
    windows: Res<Windows>,
    mut q: Query<(&LocalPlayer, &mut ExternalForce, &Transform)>,
    camera: Query<&Transform, With<Camera>>,
) {
    let window = windows.get_primary().unwrap();

    let cursor_position = window.cursor_position();
    let (_player, mut ext_force, transform) = q.single_mut();
    let ball_center = transform.translation.truncate();
    println!("cursor: {:?}", cursor_position);
    println!("ball center: {:?}", ball_center);

    if let Some(position) = cursor_position {
        // Determine the world space coords for the cursor
        let norm = Vec3::new(
            position.x - window.width() / 2.,
            position.y - window.height() / 2.,
            0.0,
        );
        let cam_transform = camera.single();
        let world_space = cam_transform.mul_vec3(norm);
        println!("world space: {:?}", world_space);
        // Change the velocity to be towards the cursor
        // let direction = (position - ball_center).normalize();
        ext_force.force = world_space.truncate() - ball_center;
        ext_force.torque = 3.;
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_follow(
    mut cam: Query<&mut Transform, (With<Camera>, Without<LocalPlayer>)>,
    q: Query<(&Transform, (With<LocalPlayer>, Without<Camera>))>,
) {
    let mut cam = cam.single_mut();
    let (transform, _player) = q.single();

    let delta = transform.translation - cam.translation;
    cam.translation += delta * 0.1;
}

#[derive(Component)]
pub struct SpeedLimit(f32);

pub fn enforce_speed_limit(q: Query<(&mut Velocity, &SpeedLimit)>) {
    for (velocity, speed_limit) in &mut q.iter() {
        let max = Vec2::new(speed_limit.0, speed_limit.0);
        velocity.linvel.clamp(-max, max);
    }
}
