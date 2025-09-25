use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
mod plugins;

use plugins::snake::snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
