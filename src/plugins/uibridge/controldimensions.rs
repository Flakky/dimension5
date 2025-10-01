use crate::plugins::snake::visualization_state::DimensionState;
use crate::plugins::snake::visualization_state::VisualizationState;
use crate::plugins::ui::dimensionblock::Axis;
use crate::plugins::ui::dimensionblock::UIDimensionBlock;
use bevy::prelude::*;

pub fn control_dimensions(
    mut visualization_state: ResMut<VisualizationState>,
    query: Query<&UIDimensionBlock>,
) {
    for uidimensionblock in query.iter() {
        match uidimensionblock.axis {
            Some(axis) => match axis {
                Axis::X => {
                    visualization_state.x =
                        DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::X)
                }
                Axis::Y => {
                    visualization_state.y =
                        DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::Y)
                }
                Axis::Z => {
                    visualization_state.z =
                        DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::Z)
                }
            },
            None => match uidimensionblock.dimension {
                1 => {
                    visualization_state.x =
                        DimensionState::Value((uidimensionblock.value * 100.0) as u8)
                }
                2 => {
                    visualization_state.y =
                        DimensionState::Value((uidimensionblock.value * 100.0) as u8)
                }
                3 => {
                    visualization_state.z =
                        DimensionState::Value((uidimensionblock.value * 100.0) as u8)
                }
                4 => {
                    visualization_state.t =
                        DimensionState::Value((uidimensionblock.value * 100.0) as u8)
                }
                5 => {
                    visualization_state.d5 =
                        DimensionState::Value((uidimensionblock.value * 100.0) as u8)
                }
                _ => unreachable!(),
            },
        }
    }
}
