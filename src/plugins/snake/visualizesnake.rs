use bevy::prelude::*;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::snake::snakecell::SnakeCell;
use crate::plugins::snake::snakecache::SnakeCache;

pub fn visualize_snake(
    snake_state: Res<SnakeState>,
    mut query: Query<(&SnakeCell, &mut Visibility)>,
    snake_cache: Res<SnakeCache>,
){
    for (snake_cell, mut visibility) in query.iter_mut() {
        
        let d4 = if snake_state.visualize_dimention_x == 4 {snake_cell.x} else {
            if snake_state.visualize_dimention_y == 4 {snake_cell.y} else {
                if snake_state.visualize_dimention_z == 4 {snake_cell.z} else {
                    snake_state.dimention4
                }
            }
        };

        let d5 = if snake_state.visualize_dimention_x == 5 {snake_cell.x} else {
            if snake_state.visualize_dimention_y == 5 {snake_cell.y} else {
                if snake_state.visualize_dimention_z == 5 {snake_cell.z} else {
                    snake_state.dimention5
                }
            }
        };

        //debug!("d4:{} - d5:{}", d4, d5);

        let snake_cells: Vec<Vec3> = snake_cache.snake_cells_map.get(&d4).unwrap().get(&d5).unwrap().clone();

        if is_cell_in_snake_cells(snake_cell, &snake_cells, &snake_state) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn is_cell_in_snake_cells(cell: &SnakeCell, snake_cells: &Vec<Vec3>, snake_state: &SnakeState) -> bool {
    for snake_cell in snake_cells {

        let mut cell_location = Vec3::new(-1.0, -1.0, -1.0);

        cell_location.x = match snake_state.visualize_dimention_x {
            1 => snake_cell.x as f32,
            2 => snake_cell.y as f32,
            3 => snake_cell.z as f32,
            4 => cell.x as f32,
            5 => cell.x as f32,
            _ => cell_location.x,
        };

        cell_location.y = match snake_state.visualize_dimention_y {
            1 => snake_cell.x as f32,
            2 => snake_cell.y as f32,
            3 => snake_cell.z as f32,
            4 => cell.y as f32,
            5 => cell.y as f32,
            _ => cell_location.y,
        };

        cell_location.z = match snake_state.visualize_dimention_z {
            1 => snake_cell.x as f32,
            2 => snake_cell.y as f32,
            3 => snake_cell.z as f32,
            4 => cell.z as f32,
            5 => cell.z as f32,
            _ => cell_location.z,
        };

        //cell_location.x = if cell_location.x < 0.0 { snake_state.dimention1 as f32 } else { cell_location.x };
        //cell_location.y = if cell_location.y < 0.0 { snake_state.dimention2 as f32 } else { cell_location.y };
        //cell_location.z = if cell_location.z < 0.0 { snake_state.dimention3 as f32 } else { cell_location.z };

        // TODO use u8
        if (cell.x as i32) == (cell_location.x as i32)
            && cell.y as i32 == (cell_location.y as i32)
            && cell.z as i32 == (cell_location.z as i32) {
            return true;
        }
    }
    return false;
}
