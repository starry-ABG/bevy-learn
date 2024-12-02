use std::time::Duration;

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
    state::commands,
};

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
struct Cursor;

#[derive(Component)]
struct BoxContent;

struct InputBoxPlugin;
impl Plugin for InputBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorTimer(Timer::new(
            Duration::from_millis(530),
            TimerMode::Repeating,
        )))
        .add_systems(Startup, init)
        .add_systems(Update, (input, animate_cursor))
        .add_systems(
            Update,
            // listen_keyboard_input_events.before(update_input_box),
            (listen_keyboard_input_events, update_input_box)
        );
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansSC.ttf");
    commands
        .spawn((
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
        ))
        .with_child((TextSpan::new(""), BoxContent))
        .with_child((TextSpan::new(""),))
        .with_child((
            TextSpan::new("|"),
            TextColor::from(Color::srgba(1., 1., 1., 1.)),
            Cursor,
        ));
}
fn input() {}

fn animate_cursor(
    time: Res<Time>,
    mut timer: ResMut<CursorTimer>,
    mut cursor: Query<&mut TextColor, With<Cursor>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut c = cursor.get_single_mut().unwrap();
        if c.0.alpha() == 1.0 {
            c.0 = Color::srgba(1., 1., 1., 0.);
        } else {
            c.0 = Color::srgba(1., 1., 1., 1.);
        }
    }
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    input_box: Single<&mut InputBox>,
) {
    let mut input_box = input_box.into_inner();
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }
        match &event.logical_key {
            Key::Character(character) => {
                input_box.content.push_str(character);
            }
            Key::Enter => {
                input_box.content.clear();
            }
            Key::Backspace => {
                input_box.content.pop();
            }
            Key::Space => {
                input_box.content.push_str(" ");
            }
            _ => continue,
        }
    }
}

fn update_input_box(
    mut commands: Commands,
    input_box: Query<(Entity, &InputBox, &Children)>,
    // input_box: Query<Entity, With<InputBox>>,
    mut conten: Query<&mut TextSpan, With<BoxContent>>,
) {
    
    // for e in input_box.iter() {
    //     println!("www{}", 1);
    // }
    // for (e, b) in input_box.iter() {
    //     println!("www{}", b.content);
    // }
    for (e, b, children) in input_box.iter() {
        let text_span = children.get(0).unwrap();
        // println!("children {}", children.len());
        if let Ok(mut c) = conten.get_mut(*text_span) {
            c.0 = b.content.clone();
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
