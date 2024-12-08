use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                resolution: bevy::window::WindowResolution::new(1280., 720.),
                title: "世纪之旅".to_string(),
                name: Some("世纪之旅".parse().unwrap()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .run();
}