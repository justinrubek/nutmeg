use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod gui;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugin(HelloPlugin)
        .add_plugin(gui::GuiPlugin::default())
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
