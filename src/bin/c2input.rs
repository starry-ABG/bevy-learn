use std::time::Duration;

use bevy::{
    ecs::event,
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::MouseButtonInput,
    },
    prelude::*,
    state::commands,
};

#[derive(Component)]
#[require(Text, Node, Interaction, CursorVisible(|| CursorVisible(true)))]
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

#[derive(Resource, Default)]
struct Focused(Option<Entity>);

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
        .insert_resource(Focused::default())
        .add_systems(Startup, init)
        // .add_systems(Update, (input, animate_cursor))
        // .add_systems(
        //     Update,
        //     // listen_keyboard_input_events.before(update_input_box),
        //     (
        //         listen_keyboard_input_events,
        //         update_input_box,
        //         listen_ime_events,
        //         handle_foucus,
        //     ),
        // )
        ;
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Single<&mut Window>) {
    window.ime_enabled = true;
    // let font = asset_server.load("fonts/NotoSansSC.ttf");
    commands
        .spawn(
            (Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(16.)),
                // margin: UiRect::all(Val::Px(32.)),
                // padding: UiRect::all(Val::Px(32.)),
                ..default()
            },
            BorderColor(Color::srgb(1., 0., 0.)),
            BackgroundColor(Color::srgb(0., 1., 0.)),
        ),
        )
        .with_child((
            InputBox {
                content: String::new(),
                cursor_position: 0,
                max_length: 50,
                is_focused: false,
                ime_text: String::new(),
                composition_range: None,
            },
            Node {
                width: Val::Px(165.),
                // width: Val::Percent(100.),
                height: Val::Px(50.),
                // height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::BLACK),
            // TextFont {
            //     font: font,
            //     font_size: 32.,
            //     ..default()
            // },
        ))
        // .with_child((TextSpan::new(""), BoxContent))
        // .with_child((TextSpan::new(""),))
        // .with_child((
        //     TextSpan::new("|"),
        //     TextColor::from(Color::srgba(1., 1., 1., 1.)),
        //     Cursor,
        // ))
        ;
}
fn input() {}

fn animate_cursor(
    time: Res<Time>,
    mut timer: ResMut<CursorTimer>,
    mut cursor: Query<&mut TextColor, With<Cursor>>,
    input_box: Query<(Entity, &Children), With<InputBox>>,
    focused: Res<Focused>,
) {
    if let Some(e) = focused.0 {
        if timer.0.tick(time.delta()).just_finished() {
            let (_, children) = input_box.get(e).unwrap();
            let cursor_entity = children[2];
            let mut c = cursor.get_mut(cursor_entity).unwrap();
            if c.0.alpha() == 1.0 {
                c.0 = Color::srgba(1., 1., 1., 0.);
            } else {
                c.0 = Color::srgba(1., 1., 1., 1.);
            }
        }
    }
}

fn listen_keyboard_input_events(
    focused: Res<Focused>,
    mut events: EventReader<KeyboardInput>,
    mut input_box: Query<&mut InputBox>,
) {
    if let Some(e) = focused.0 {
        let mut input_box = input_box.get_mut(e).unwrap();
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
}

fn handle_foucus(
    mut f: ResMut<Focused>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    input_box: Query<(Entity, &Interaction), With<InputBox>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for (e, i) in input_box.iter() {
            match *i {
                Interaction::Hovered => {
                    println!("hovered");
                }
                Interaction::Pressed => {
                    println!("pressed");
                    f.0 = Some(e);
                    return;
                }
                Interaction::None => {
                    println!("None");
                }
            }
        }
        f.0 = None;
    }
}

fn update_input_box(
    focused: Res<Focused>,
    input_box: Query<(Entity, &InputBox, &Children)>,
    // input_box: Query<Entity, With<InputBox>>,
    mut conten: Query<&mut TextSpan, With<BoxContent>>,
) {
    if let Some(e) = focused.0 {
        let (e, b, children) = input_box.get(e).unwrap();
        let text_span = children.first().unwrap();
        // let text_span = children.get(0).unwrap();
        // println!("children {}", children.len());
        if let Ok(mut c) = conten.get_mut(*text_span) {
            c.0 = b.content.clone();
        }
    }
}

fn listen_ime_events(
    focused: Res<Focused>,
    mut events: EventReader<Ime>,
    mut input_box: Query<&mut InputBox>,
) {
    if let Some(e) = focused.0 {
        let mut input_box = input_box.get_mut(e).unwrap();
        for event in events.read() {
            match event {
                Ime::Commit { value, .. } => {
                    input_box.content.push_str(value);
                }
                _ => (),
            }
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
