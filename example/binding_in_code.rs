use bevy::prelude::*;
use bevy_app::AppExit;
use bevy_app::Events;
use bevy_ecs::ResMut;
use bevy_prototype_input_map::{inputmap::InputMap, InputMapPlugin, axis::Axis, phase::Phase};

fn main() {
    println!("Input Map Binding In Code Example");
    App::build()
        .add_default_plugins()
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}

fn setup(
    mut input_map: ResMut<InputMap>,
) {
    // keyboard
    input_map
    .bind_keyboard_pressed(KeyCode::Space, "JUMP")
    .bind_keyboard_pressed(KeyCode::Return, "SHOOT")

    .bind_keyboard_pressed(KeyCode::Escape, "QUIT_APP")

    // mouse
    .bind_mouse_button_pressed(MouseButton::Left, "SHOOT")
    .bind_mouse_button_pressed(MouseButton::Right, "JUMP")

    .bind_mouse_motion(Axis::YNegative, "AIM_UP")
    .bind_mouse_motion(Axis::YPositive, "AIM_DOWN")
    .bind_mouse_motion(Axis::XNegative, "AIM_LEFT")
    .bind_mouse_motion(Axis::XPositive, "AIM_RIGHT")

    // set event phase
    .set_event_phase("QUIT_APP", Phase::OnEnded)
    .set_event_phase("SHOOT", Phase::OnBegin)

    // dead zone
    .set_dead_zone("AIM_UP", 0.1)
    .set_dead_zone("AIM_DOWN", 0.1)
    .set_dead_zone("AIM_LEFT", 0.1)
    .set_dead_zone("AIM_RIGHT", 0.1)

    // custom strength curve function
    .set_strength_curve_function("AIM_UP", |x  | -> f32 { x.powi(2)})
    .set_strength_curve_function("AIM_DOWN", |x  | -> f32 { x.powi(2) })
    .set_strength_curve_function("AIM_LEFT", |x  | -> f32 { x.powi(2) })
    .set_strength_curve_function("AIM_RIGHT", |x  | -> f32 { x.powi(2) });

    println!("{}", input_map.get_bindings_as_json().unwrap());
}

fn action_system(input_map: Res<InputMap>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if input_map.is_action_active("JUMP") {
        println!("Jumping...");
    }

    if input_map.is_action_active("SHOOT") {
        println!("Bang");
    }

    if input_map.is_action_active("AIM_UP") {
        println!(
            "AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP")
        );
    }

    if input_map.is_action_active("AIM_DOWN") {
        println!(
            "AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN")
        );
    }

    if input_map.is_action_active("AIM_LEFT") {
        println!(
            "AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT")
        );
    }

    if input_map.is_action_active("AIM_RIGHT") {
        println!(
            "AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT")
        );
    }

    if input_map.is_action_active("QUIT_APP") {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}
