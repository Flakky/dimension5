use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct SnakeState {
    pub dimention1: u8,
    pub dimention2: u8,
    pub dimention3: u8,
    pub dimention4: u8,
    pub dimention5: u8,

    pub visualize_dimention_x: u8,
    pub visualize_dimention_y: u8,
    pub visualize_dimention_z: u8,
}
