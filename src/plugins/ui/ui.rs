use bevy::prelude::*;
use crate::plugins::ui::dimentionblock::create_dimention_blocks;

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_dimention_blocks);
    }
}

