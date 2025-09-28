use bevy::prelude::*;
use crate::plugins::snake::visualizesnake::visualize_snake;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::snake::snakecell::create_snake_cells;
use crate::plugins::snake::snakecell::GRID_SIZE;

/// Snake plugin for the Bevy game engine
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_snake)
            .add_systems(Startup, spawn_axis_meshes)
            .add_systems(Startup, create_snake_cells)
            .insert_resource(SnakeState::default())
            .add_systems(Update, visualize_snake)
            ;
    }
}

fn setup_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(GRID_SIZE as f32 / 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(GRID_SIZE as f32 / 2.0, 0.0, GRID_SIZE as f32 / 2.0)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 100.0),
    ));
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-20.5, 40.5, 90.0)
            .looking_at(Vec3::new(GRID_SIZE as f32 / 2.0, GRID_SIZE as f32 / 2.0, GRID_SIZE as f32 / 2.0), Vec3::Y),
    ));
}

fn spawn_axis_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // X
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(GRID_SIZE as f32, 0.1, 0.1))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.3, 0.3))),
        Transform::from_xyz(GRID_SIZE as f32 / 2.0, 0.0, 0.0),
    ));
    // Y
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.1, GRID_SIZE as f32, 0.1))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 1.0, 0.3))),
        Transform::from_xyz(0.0, GRID_SIZE as f32 / 2.0, 0.0),
    ));
    // Z
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, GRID_SIZE as f32))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 1.0))),
        Transform::from_xyz(0.0, 0.0, GRID_SIZE as f32 / 2.0),
    ));
}