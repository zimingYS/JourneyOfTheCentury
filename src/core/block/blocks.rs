use bevy::prelude::*;
use bevy::render::render_resource::Texture;

pub struct BlockSize{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlockType{
    Dirt,
}

#[derive(Resource)]
pub struct Block{
    pub id: String,
    pub size: Mesh3d,
    pub texture: Handle<Image>,
    pub hardness: f32,
    pub block_type: BlockType,
    pub is_interactive: bool,
}
impl Block{
    pub fn new(id: String,
               size: BlockSize,
               texture: Handle<Image>,
               hardness: f32,
               block_type: BlockType,
               is_interactive: bool,
               mut meshes: ResMut<Assets<Mesh>>,
    ) -> Self{

        Self{
            id,
            size :Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            texture,
            hardness,
            block_type,
            is_interactive,
        }
    }
}