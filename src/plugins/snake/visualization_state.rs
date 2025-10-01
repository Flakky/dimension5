use bevy::prelude::*;

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Default)]
pub enum Dimension {
    #[default]
    X,
    Y,
    Z,
    T,
    D5,
}

impl Dimension {
    pub const fn project(&self, x: u8, y: u8, z: u8, t: u8, d5: u8) -> u8 {
        match self {
            Dimension::X => x,
            Dimension::Y => y,
            Dimension::Z => z,
            Dimension::T => t,
            Dimension::D5 => d5,
        }
    }

    pub const fn from_u8(value: u8) -> Self {
        match value {
            1 => Dimension::X,
            2 => Dimension::Y,
            3 => Dimension::Z,
            4 => Dimension::T,
            5 => Dimension::D5,
            _ => unreachable!(),
        }
    }
}

#[derive(Resource, Debug)]
pub struct VisualizationState {
    pub x: DimensionState,
    pub y: DimensionState,
    pub z: DimensionState,
    pub t: DimensionState,
    pub d5: DimensionState,
}

impl Default for VisualizationState {
    fn default() -> Self {
        Self {
            x: DimensionState::Axis(Axis::X),
            y: DimensionState::Axis(Axis::Y),
            z: DimensionState::Axis(Axis::Z),
            t: DimensionState::Value(0),
            d5: DimensionState::Value(0),
        }
    }
}

impl DimensionState {
    pub fn resolve(&self, x: u8, y: u8, z: u8) -> u8 {
        match self {
            DimensionState::Axis(Axis::X) => x,
            DimensionState::Axis(Axis::Y) => y,
            DimensionState::Axis(Axis::Z) => z,
            DimensionState::Value(value) => *value,
        }
    }
}

#[derive(Debug)]
pub enum DimensionState {
    Axis(Axis), // Visualize
    Value(u8),  // Fixed
}
