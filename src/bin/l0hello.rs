use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello).run();
}

fn hello() {
    println!("hello");
}