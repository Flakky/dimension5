use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SnakeState {
    pub dimention1: u8,
    pub dimention2: u8,
    pub dimention3: u8,
    pub dimention4: u8,
    pub dimention5: u8,

    pub visualize_dimentionX: u8,
    pub visualize_dimentionY: u8,
    pub visualize_dimentionZ: u8,
}
