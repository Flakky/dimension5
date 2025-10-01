use bevy::prelude::*;

mod plugins;

use plugins::snake::snake::SnakePlugin;
use plugins::ui::ui::UI;
use plugins::uibridge::uibridge::UIBridge;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakePlugin)
        .add_plugins(UI)
        .add_plugins(UIBridge);

    #[cfg(debug_assertions)]
    {
        use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
        
        app
            .add_plugins(FpsOverlayPlugin::default());
    }

    app.run();
}

