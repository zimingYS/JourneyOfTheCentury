mod camera;
mod setup;

use bevy::prelude::*;
use crate::camera::move_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                //窗口分辨率
                resolution: bevy::window::WindowResolution::new(1280., 720.),
                //窗口标题
                title: "世纪之旅".to_string(),
                //窗口名称
                name: Some("世纪之旅".parse().unwrap()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup::setup)
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Update,move_camera)
        .add_systems(Startup, setup::cursor_control)
        .run();
}
