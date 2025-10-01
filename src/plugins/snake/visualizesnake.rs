use bevy::prelude::*;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::snake::snakecell::SnakeCell;
use rand::{Rng, SeedableRng};
use crate::plugins::snake::snakecell::GRID_SIZE;
use std::collections::HashMap;

static SNAKE_LENGTH: u8 = 5;
static SWITCH_PERIOD: u8 = 5;

pub fn visualize_snake(
    snake_state: Res<SnakeState>,
    mut query: Query<(&SnakeCell, &mut Visibility)>,
) {
    let visualize_dimention4: bool = 
    if snake_state.visualize_dimention_x == 4 
    || snake_state.visualize_dimention_y == 4 
    || snake_state.visualize_dimention_z == 4 {
        true
    } else {
        false
    };

    let visualize_dimention5: bool = 
    if snake_state.visualize_dimention_x == 5 
    || snake_state.visualize_dimention_y == 5 
    || snake_state.visualize_dimention_z == 5 {
        true
    } else {
        false
    };
    
    let mut snake_cells_map: HashMap<u8, HashMap<u8, Vec<Vec3>>> = HashMap::new();

    if visualize_dimention4 {
        for i in 0..100 {
            snake_cells_map.insert(i, HashMap::new());
            if visualize_dimention5 {
                for j in 0..100 {
                    snake_cells_map.get_mut(&i).unwrap().insert(j, get_snake_cells(
                        i, 
                        SWITCH_PERIOD, 
                        GRID_SIZE, 
                        j,
                        SNAKE_LENGTH));
                }
            }
            else {
                snake_cells_map.get_mut(&i).unwrap().insert(snake_state.dimention5, get_snake_cells(
                    i, 
                    SWITCH_PERIOD, 
                    GRID_SIZE, 
                    snake_state.dimention5,
                    SNAKE_LENGTH));
            }
        }
    }
    else if visualize_dimention5 {
        snake_cells_map.insert(snake_state.dimention4, HashMap::new());
        for i in 0..100 {
            snake_cells_map.get_mut(&snake_state.dimention4).unwrap().insert(i, get_snake_cells(
                snake_state.dimention4, 
                SWITCH_PERIOD, 
                GRID_SIZE, 
                i,
                SNAKE_LENGTH));
        }
    }
    else {
        snake_cells_map.insert(snake_state.dimention4, HashMap::new());
        snake_cells_map.get_mut(&snake_state.dimention4).unwrap().insert(snake_state.dimention5, get_snake_cells(
            snake_state.dimention4, 
            SWITCH_PERIOD, 
            GRID_SIZE, 
            snake_state.dimention5,
            SNAKE_LENGTH));
    }
  
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

        // println!("d4:{} - d5:{}", d4, d5);

        let snake_cells: Vec<Vec3> = snake_cells_map.get(&d4).unwrap().get(&d5).unwrap().clone();

        if visualize_dimention4 {
            // println!("snake_cells: {:?} d4:{} - d5:{}", snake_cells, d4, d5);
        }

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

fn get_snake_cells(time: u8, switch_period: u8, grid_size: u8, dimension_seed: u8, snake_length: u8) -> Vec<Vec3> {
    let mut t: u8 = 0;
    let mut direction: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let mut last_direction: Vec3 = direction;
    let mut position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    loop {
        let new_time = u8::min(t + switch_period, time);

        let delta = new_time - t;

        position = position + (direction.normalize() * (delta as f32));

        t = t + delta;

        // TODO: Check for bounds to switch direction

        // println!("t:{} - tm:{} - dir:{} - ldir:{} - d:{} - p:{}", t, time, direction, last_direction, delta, position);

        if t >= time {
            let mut snake_cells: Vec<Vec3> = Vec::new();
            let mut cell_position: Vec3 = position;
            for i in 0..snake_length {
                snake_cells.push(cell_position);
                let backward_direction = if i < delta { -direction } else { -last_direction };
                cell_position = cell_position + backward_direction;
            }
            return snake_cells;
        }
        else {
            last_direction = direction;
            direction = random_direction(direction, dimension_seed+t, position, grid_size);
        }
    }
}

fn random_direction(direction: Vec3, dimension_seed: u8, position: Vec3, grid_size: u8) -> Vec3 {
    let possible_directions = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];

    let possible_directions = 
    possible_directions.iter().filter(|&d| {
        !( // Exclude unsuitable directions
            d == &direction 
            || d == &(-direction)
            || (position.x as u8 <= 0 && d.x == -1.0)
            || (position.x as u8 >= grid_size - 1 && d.x == 1.0 )
            || (position.y as u8 <= 0 && d.y == -1.0)
            || (position.y as u8 >= grid_size - 1 && d.y == 1.0)
            || (position.z as u8 <= 0 && d.z == -1.0)
            || (position.z as u8 >= grid_size - 1 && d.z == 1.0)
        )
    }).collect::<Vec<&Vec3>>();

    let mut rng = rand::rngs::StdRng::seed_from_u64(dimension_seed as u64);
    let index = rng.random_range(0..possible_directions.len());

    return possible_directions[index].clone();
}