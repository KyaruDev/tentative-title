use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{resources::board_options::BoardOptions, BoardPlugin};

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();

    let mut primary_window = Window::default();
    primary_window.resolution = WindowResolution::new(700.0, 800.0);
    primary_window.title = "tentative-title 0.1.0-alpha".to_string();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(primary_window),
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    }));

    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        tile_size: board_plugin::resources::board_options::TileSize::Fixed(20.0),
        ..Default::default()
    });
    app.add_plugins(BoardPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, camera_setup);
    app.run();
}
