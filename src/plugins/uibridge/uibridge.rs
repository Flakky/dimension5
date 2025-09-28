use bevy::prelude::*;
use crate::plugins::uibridge::controldimensions::control_dimensions;

pub struct UIBridge;

impl Plugin for UIBridge {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_dimensions);
    }
}