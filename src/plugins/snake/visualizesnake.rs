use bevy::prelude::*;
use crate::plugins::snake::snakestate::SnakeState;
use crate::plugins::snake::snakecell::SnakeCell;
use rand::random;

static SNAKE_LENGTH: u8 = 10;

pub fn visualize_snake(
    mut commands: Commands,
    snake_state: Res<SnakeState>,
    query: Query<Entity, With<SnakeCell>>,
) {
    for entity in query.iter() {
        let visibility: Visibility = if random::<bool>() { Visibility::Visible } else { Visibility::Hidden };
        commands.entity(entity).insert(visibility);
    }
}