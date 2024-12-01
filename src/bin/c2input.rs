use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
#[require(Text, CursorVisible(|| CursorVisible(true)))]
struct InputBox {
    content: String,
    cursor_position: usize,
    max_length: usize,
    is_focused: bool,
    ime_text: String,
    composition_range: Option<(usize, usize)>,
}

#[derive(Resource)]
struct CursorTimer(Timer);

#[derive(Component, Default)]
struct CursorVisible(bool);

#[derive(Component)]
#[require()]
struct Cursor;

struct InputBoxPlugin;
impl Plugin for InputBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorTimer(Timer::new(
            Duration::from_millis(530),
            TimerMode::Repeating,
        )))
        .add_systems(Startup, init)
        .add_systems(Update, (input, animate_cursor));
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansSC.ttf");
    commands.spawn((
        InputBox {
            content: String::new(),
            cursor_position: 0,
            max_length: 50,
            is_focused: false,
            ime_text: String::new(),
            composition_range: None,
        },
        Text::default(),
        TextFont {
            font: font,
            font_size: 32.,
            ..default()
        },
    )).with_child((
        TextSpan::new("con"),
    )).with_child((
        TextSpan::new("con"),
    )).with_child((
        TextSpan::new("|"),
        TextColor::from(Color::srgba(1., 1., 1., 1.)),
        Cursor
    ))
    ;
}
fn input() {}

fn animate_cursor(time: Res<Time>, mut timer: ResMut<CursorTimer>, mut cursor: Query<&mut TextColor, With<Cursor>>) {

    if timer.0.tick(time.delta()).just_finished() {
        let mut c = cursor.get_single_mut().unwrap();
        if c.0.alpha() == 1.0 {
            c.0 = Color::srgba(1., 1., 1., 0.);
            println!("vvv");
        } else {
            c.0 = Color::srgba(1., 1., 1., 1.);
            println!("hhh");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputBoxPlugin)
        .add_systems(Startup, init_app)
        .run();
}

fn init_app(mut commands: Commands) {
    commands.spawn(Camera2d);
}
