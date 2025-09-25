use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SnakeState {
    pub dimention1: u8,
    pub dimention2: u8,
    pub dimention3: u8,
    pub dimention4: u8,
    pub dimention5: u8,

    pub visualize_dimention1: bool,
    pub visualize_dimention2: bool,
    pub visualize_dimention3: bool,
    pub visualize_dimention4: bool,
    pub visualize_dimention5: bool,
}
