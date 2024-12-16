use std::f32::consts::FRAC_PI_2;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Component)]
struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

pub fn move_camera(time: Res<Time>,
                   keyboard_input: Res<ButtonInput<KeyCode>>,
                   mut query: Query<&mut Transform, With<Camera>>,
                   mouse_motion: Res<AccumulatedMouseMotion>,
                   mut cursor_visible: Local<bool>,
                   mut primary: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut transform = query.single_mut();
    let speed = 0.03;


    //键盘控制移动
    let forward = transform.forward();
    let right = transform.right();
    let up = Vec3::Y;

    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation += forward * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation -= forward * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation -= right * speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation += right * speed;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        transform.translation -= up * speed;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        transform.translation += up * speed;
    }
    let mut primary_window = primary.single_mut();

    //按下ESC松开鼠标固定
    if keyboard_input.just_released(KeyCode::Escape) {
        *cursor_visible = !*cursor_visible;
    }

    //将光标固定在窗口内并取消光标显示
    if !*cursor_visible {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;

        //鼠标控制视角移动
        let delta = mouse_motion.delta;

        if delta != Vec2::ZERO {
            // 计算鼠标增量对应的旋转变化
            let sensitivity = speed;  // 灵敏度调整，可以根据需要修改

            // 计算yaw和pitch增量
            let delta_yaw = -delta.x * sensitivity * 0.1;
            let delta_pitch = -delta.y * sensitivity * 0.1;

            // 获取当前的旋转角度
            let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

            // 更新yaw和pitch
            yaw += delta_yaw;
            pitch = (pitch + delta_pitch).clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01); // 限制pitch角度

            // 更新旋转
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    } else {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }
}