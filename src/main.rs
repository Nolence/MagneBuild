use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MagneBuild".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..Default::default()
        }))
        // .add_plugin(emergence_lib::ui::UiPlugin)
        .run();
}
