use bevy::prelude::*;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::ui::dimensionblock::UIDimensionBlock;
use crate::plugins::ui::dimensionblock::Axis;


pub fn control_dimensions(
    mut snake_state: ResMut<SnakeState>,
    query: Query<&UIDimensionBlock>,
) {
    for uidimensionblock in query.iter() {
        match uidimensionblock.dimension {
            1 => snake_state.dimention1 = (uidimensionblock.value * 100.0) as u8,
            2 => snake_state.dimention2 = (uidimensionblock.value * 100.0) as u8,
            3 => snake_state.dimention3 = (uidimensionblock.value * 100.0) as u8,
            4 => snake_state.dimention4 = (uidimensionblock.value * 100.0) as u8,
            5 => snake_state.dimention5 = (uidimensionblock.value * 100.0) as u8,
            _ => {} // Handle all other values
        }

        match uidimensionblock.axis {
            Axis::X => snake_state.visualize_dimention_x = uidimensionblock.dimension,
            Axis::Y => snake_state.visualize_dimention_y = uidimensionblock.dimension,
            Axis::Z => snake_state.visualize_dimention_z = uidimensionblock.dimension,
            Axis::None => {} // Handle the None case
        }
    }
}