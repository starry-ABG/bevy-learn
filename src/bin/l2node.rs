use bevy::{prelude::*, render::view::window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, init_ui)
        .run();
}

fn init_ui(mut commands: Commands, window: Single<&Window>) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                // width: Val::Px(window.width()),
                // height: Val::Px(window.height()),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0., 0.)),
        ))
        .with_child((
            Node {
                width: Val::Px(150.),
                height: Val::Px(65.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::srgb(0., 1., 0.))
        ));
}
