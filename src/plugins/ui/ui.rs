use bevy::prelude::*;
use crate::plugins::ui::dimentionblock::create_dimention_blocks;
use crate::plugins::ui::dimentionblock::update_value_selector;
use crate::plugins::ui::dimentionblock::control_dimention_value_selector;
use crate::plugins::ui::dimentionblock::select_axis;
use crate::plugins::ui::dimentionblock::update_axis_selector;

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_dimention_blocks);
        app.add_systems(Update, update_value_selector);
        app.add_systems(Update, control_dimention_value_selector);
        app.add_systems(Update, select_axis);
        app.add_systems(Update, update_axis_selector);
    }
}

