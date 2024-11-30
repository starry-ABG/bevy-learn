use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, (run, log))
        .run();
}

#[derive(Component)]
struct Pos {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);

fn init(mut commands: Commands) {
    commands.spawn((Name("God".to_string()), Pos { x: 0., y: 0. }));
}

fn run(mut query: Query<(&mut Pos, &Name)>) {
    for (mut p, n) in &mut query {
        p.x += 10.;
    }
}

fn log(query: Query<(&Name, &Pos)>) {
    for (n, p) in &query {
        println!("{} {}", n.0, p.x);
    }
}
