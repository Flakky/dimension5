use crate::plugins::snake::snakecache::SnakeCache;
use crate::plugins::snake::snakecell::SnakeCell;
use crate::plugins::snake::visualization_state::Axis;
use crate::plugins::snake::visualization_state::DimensionState;
use crate::plugins::snake::visualization_state::VisualizationState;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn visualize_snake(
    visualization_state: Res<VisualizationState>,
    mut query: Query<(&SnakeCell, &mut Visibility)>,
    snake_cache: Res<SnakeCache>,
) {
    info!("visualization state: {:?}", visualization_state);

    for (world_snake, mut visibility) in query.iter_mut() {
        let d4 = visualization_state
            .t
            .resolve(world_snake.x, world_snake.y, world_snake.z);

        let d5 = visualization_state
            .d5
            .resolve(world_snake.x, world_snake.y, world_snake.z);

        //debug!("d4:{} - d5:{}", d4, d5);

        let snake_cells = snake_cache
            .snake_cells_map
            .get(&d4)
            .unwrap()
            .get(&d5)
            .unwrap();

        if is_cell_in_snake_cells(world_snake, &snake_cells, &visualization_state) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn is_cell_in_snake_cells(
    world_cell: &SnakeCell,
    snake_cells: &HashSet<SnakeCell>,
    visualization_state: &VisualizationState,
) -> bool {
    for snake_cell in snake_cells {
        let x = match visualization_state.x {
            DimensionState::Axis(Axis::X) => world_cell.x == snake_cell.x,
            DimensionState::Axis(Axis::Y) => world_cell.y == snake_cell.x,
            DimensionState::Axis(Axis::Z) => world_cell.z == snake_cell.x,
            DimensionState::Value(value) => snake_cell.x == value,
        };

        let y = match visualization_state.y {
            DimensionState::Axis(Axis::X) => world_cell.x == snake_cell.y,
            DimensionState::Axis(Axis::Y) => world_cell.y == snake_cell.y,
            DimensionState::Axis(Axis::Z) => world_cell.z == snake_cell.y,
            DimensionState::Value(value) => snake_cell.y == value,
        };

        let z = match visualization_state.z {
            DimensionState::Axis(Axis::X) => world_cell.x == snake_cell.z,
            DimensionState::Axis(Axis::Y) => world_cell.y == snake_cell.z,
            DimensionState::Axis(Axis::Z) => world_cell.z == snake_cell.z,
            DimensionState::Value(value) => snake_cell.z == value,
        };

        if x && y && z {
            return true;
        }
    }
    return false;
}
