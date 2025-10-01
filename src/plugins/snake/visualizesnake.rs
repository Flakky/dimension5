use crate::plugins::snake::snakecache::SnakeCache;
use crate::plugins::snake::snakecell::SnakeCell;
use crate::plugins::snake::visualization_state::Dimension;
use crate::plugins::snake::visualization_state::VisualizationState;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn visualize_snake(
    snake_state: Res<VisualizationState>,
    mut query: Query<(&SnakeCell, &mut Visibility)>,
    snake_cache: Res<SnakeCache>,
) {
    for (snake_cell, mut visibility) in query.iter_mut() {
        let d4 = match (
            &snake_state.visualize_dimention_x,
            &snake_state.visualize_dimention_y,
            &snake_state.visualize_dimention_z,
        ) {
            (Dimension::T, _, _) => snake_cell.x,
            (_, Dimension::T, _) => snake_cell.y,
            (_, _, Dimension::T) => snake_cell.z,
            _ => snake_state.t,
        };

        let d5 = match (
            &snake_state.visualize_dimention_x,
            &snake_state.visualize_dimention_y,
            &snake_state.visualize_dimention_z,
        ) {
            (Dimension::D5, _, _) => snake_cell.x,
            (_, Dimension::D5, _) => snake_cell.y,
            (_, _, Dimension::D5) => snake_cell.z,
            _ => snake_state.d5,
        };

        //debug!("d4:{} - d5:{}", d4, d5);

        let snake_cells = snake_cache
            .snake_cells_map
            .get(&d4)
            .unwrap()
            .get(&d5)
            .unwrap();

        if is_cell_in_snake_cells(snake_cell, &snake_cells, &snake_state) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn is_cell_in_snake_cells(
    cell: &SnakeCell,
    snake_cells: &HashSet<SnakeCell>,
    snake_state: &VisualizationState,
) -> bool {
    for snake_cell in snake_cells {
        let mut cell_location = SnakeCell { x: 0, y: 0, z: 0 };

        cell_location.x = snake_state.visualize_dimention_x.project(
            snake_cell.x,
            snake_cell.y,
            snake_cell.z,
            cell.x,
            cell.x,
        );

        cell_location.y = snake_state.visualize_dimention_y.project(
            snake_cell.x,
            snake_cell.y,
            snake_cell.z,
            cell.y,
            cell.y,
        );

        cell_location.z = snake_state.visualize_dimention_z.project(
            snake_cell.x,
            snake_cell.y,
            snake_cell.z,
            cell.z,
            cell.z,
        );

        if cell == &cell_location {
            return true;
        }
    }
    return false;
}
