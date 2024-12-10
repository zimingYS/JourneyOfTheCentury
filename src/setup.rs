use std::slice::Windows;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //create a cube
    //创建一个立方体
    for i in 0..10{
        for j in 0..10{
            commands.spawn((
                //大小
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                //纹理颜色
                MeshMaterial3d(materials.add(Color::srgb_u8(100 + (i*10), 100 + (j*10), 100))),
                //坐标
                Transform::from_xyz(i as f32, 0.0, j as f32),
            ));
        }
    }
    // light
    //设置光照
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));
}

pub fn cursor_control(
    mut primary: Query<&mut Window, With<PrimaryWindow>>,
){
    let mut primary_window = primary.single_mut();
    //将光标固定在窗口内并取消光标显示
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}