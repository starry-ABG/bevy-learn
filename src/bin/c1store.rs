use bevy::{ecs::event, input::keyboard::{Key, KeyboardInput}, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_ui)
        .add_systems(Update, (input, toggle_ime, listen_keyboard_input_events, listen_ime_events))
        .run();
}

// UI只是一个位于窗口中间的输入框
// 可以输入任何文字
// bevy中有输入框吗？
fn init_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Single<&mut Window>) {
    commands.spawn(Camera2d);
    let font = asset_server.load("fonts/NotoSansSC.ttf");
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font,
            font_size: 100.,
            ..default()
        }
    ));
    window.ime_enabled = true;
    // commands
    //     .spawn((
    //         Text::default(),
    //         Node {
    //             position_type: PositionType::Absolute,
    //             top: Val::Px(12.0),
    //             left: Val::Px(12.0),
    //             ..default()
    //         },
    //     ))
    //     .with_children(|p| {
    //         p.spawn((
    //             TextSpan::new("等待输入"),
    //             TextFont {
    //                 font: font.clone(),
    //                 ..default()
    //             }
    //         ));
    //     });
}

fn input() {}

fn toggle_ime(
    input: Res<ButtonInput<MouseButton>>,
    mut window: Single<&mut Window>,
    edit_text: Single<&Transform,With<Text2d>>
    // status_text: Single<Entity, (With<Text2d>, With<TextFont>)>,
    // mut ui_writer: TextUiWriter
) {
    if input.just_pressed(MouseButton::Left) {
        // window.ime_position = Vec2::new(edit_text.translation.x, edit_text.translation.y);
        window.ime_position = window.cursor_position().unwrap();
        // window.ime_enabled = !window.ime_enabled;
        // edit_text.clear();
        // edit_text.push_str(&format!("{}", window.ime_enabled));
        // *ui_writer.text(*status_text, 0) = format!("{}\n", window.ime_enabled);

    }

}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    edit_text: Single<&mut Text2d>
) {
    let mut text = edit_text.into_inner();
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }
        match &event.logical_key {
            Key::Character(character) => {
                text.push_str(character);
            },
            Key::Enter => {
                text.clear();
            },
            Key::Backspace => {
                text.pop();
            },
            Key::Space => {
                text.push_str(" ");
            },
            _ => continue
        }
    }
}

fn listen_ime_events(
    mut events: EventReader<Ime>,
    mut edit_text: Single<&mut Text2d>
) {
    for event in events.read() {
        match event {
           Ime::Commit { value , ..} => {
            edit_text.push_str(value);
           },
           _ => ()
        }
    }
}
