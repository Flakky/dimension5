use bevy::prelude::*;

pub static GRID_SIZE: u8 = 30;

#[derive(Component, Debug, Hash, PartialEq, Eq)]
pub struct SnakeCell{
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub fn create_snake_cells(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_xyz(
                        x as f32,
                        y as f32,
                        z as f32,
                    ),
                    Visibility::Visible,
                    SnakeCell { x, y, z },
                ));
            }
        }
    }
}