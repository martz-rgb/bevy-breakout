use bevy::{prelude::*, render::define_atomic_id, window::WindowResolution};

fn main() {
    let mut app = App::new();

    app.add_plugins(bevy::DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Breakout".to_string(),
            resolution: WindowResolution::new(
                breakout::graphics::WINDOW_WIDTH,
                breakout::graphics::WINDOW_HEIGHT,
            ),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(breakout::graphics::Graphics)
    .add_plugins(breakout::gameplay::Gameplay);

    app.run();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, setup)
    //     .run();
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
// commands.spawn(Camera2dBundle::default());
// commands.spawn(SpriteBundle {
//     texture: asset_server.load("32.png"),
//     ..default()
// });
// }
