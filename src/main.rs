mod core;

use bevy::prelude::*;
use crate::core::camera;
use crate::core::setup;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup::setup,camera::setup_camera))
        .add_systems(Update,  (camera::move_camera))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}
