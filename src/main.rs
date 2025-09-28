use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
mod plugins;

use plugins::snake::snake::SnakePlugin;
use plugins::ui::ui::UI;
use plugins::uibridge::uibridge::UIBridge;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakePlugin)
        .add_plugins(UI)
        .add_plugins(UIBridge)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
