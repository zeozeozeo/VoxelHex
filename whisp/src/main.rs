mod ui;

use crate::ui::setup_ui;
use bevy::{prelude::*, render::view::RenderLayers, window::WindowPlugin};
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            //voxelhex::raytracing::RenderBevyPlugin::<u32>::new(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            UiLunexPlugins,
            // UiLunexDebugPlugin::<1, 2>,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[0, 1]),
    ));

    setup_ui(&mut commands, &asset_server);
}
