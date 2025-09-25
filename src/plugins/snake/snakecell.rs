use bevy::prelude::*;

static GRID_SIZE: u8 = 30;
static CELL_SIZE: f32 = 1.0;

#[derive(Component)]
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
                    Mesh3d(meshes.add(Cuboid::new(CELL_SIZE, CELL_SIZE, CELL_SIZE))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_xyz(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        z as f32 * CELL_SIZE,
                    ),
                    SnakeCell { x, y, z },
                ));
            }
        }
    }
}