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
        let value = match uidimensionblock.axis {
            Some(axis) => match axis {
                Axis::X => {
                    DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::X)
                }
                Axis::Y => {
                    DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::Y)
                }
                Axis::Z => {
                    DimensionState::Axis(crate::plugins::snake::visualization_state::Axis::Z)
                }
            },
            None => DimensionState::Value((uidimensionblock.value * 100.0) as u8),
        };

        match uidimensionblock.dimension {
            1 => visualization_state.x = value,
            2 => visualization_state.y = value,
            3 => visualization_state.z = value,
            4 => visualization_state.t = value,
            5 => visualization_state.d5 = value,
            _ => unreachable!(),
        }
    }
}
