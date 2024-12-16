use bevy::prelude::*;
use crate::core::block::blocks::{Block, BlockSize, BlockType};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        //大小
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        //纹理颜色
        MeshMaterial3d(materials.add(Color::srgb_u8(100,100,100))),
        //坐标
        Transform::from_xyz(1., 0.0, 1.),
    ));

    let block = Block::new(
        "dirt_block".to_string(),
        BlockSize { x: 1.0, y: 1.0, z: 1.0 },
        asset_server.load("test\\cube-diffuse.png"),
        1.0,
        BlockType::Dirt,
        false,
        meshes,
    );

    for i in 0..10{
        for j in 0..10{
            commands.spawn(PbrBundle{
                mesh: Mesh3d::from(block.size.0.clone()),
                material: MeshMaterial3d::from(materials.add(StandardMaterial {
                    base_color_texture: Some(block.texture.clone()),
                    ..default()
                })),
                transform: Transform::from_xyz(i as f32,0.,j as f32),
                ..default()
            }
            );
        }
    }

    // 设置光照
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}