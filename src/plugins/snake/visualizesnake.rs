use bevy::prelude::*;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::snake::snakecell::SnakeCell;
use rand::{Rng, SeedableRng};
use crate::plugins::snake::snakecell::GRID_SIZE;

static SNAKE_LENGTH: u8 = 10;
static SWITCH_PERIOD: u8 = 10;

pub fn visualize_snake(
    mut commands: Commands,
    mut snake_state: ResMut<SnakeState>,
    query: Query<(Entity, &SnakeCell)>,
) {
    let time: u8 = snake_state.dimention4;
    let snake_cells = get_snake_cells(
        time, 
        SWITCH_PERIOD, 
        GRID_SIZE, 
        snake_state.dimention5,
        SNAKE_LENGTH);
        
    for (entity, snake_cell) in query.iter() {
        let visibility = if is_cell_in_snake_cells(snake_cell, &snake_cells) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        commands.entity(entity).insert(visibility);
    }
}

fn is_cell_in_snake_cells(cell: &SnakeCell, snake_cells: &Vec<Vec3>) -> bool {
    for snake_cell in snake_cells {
        // TODO use u8
        if (snake_cell.x as i32) == (cell.x as i32)
            && snake_cell.y as i32 == (cell.y as i32)
            && snake_cell.z as i32 == (cell.z as i32) {
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