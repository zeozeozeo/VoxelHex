mod components;
mod ui_behavior;
mod ui_layout;

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
                    title: "Whisp - Press g to hide/show UI".to_string(),
                    ..default()
                }),
                ..default()
            }),
            //voxelhex::raytracing::RenderBevyPlugin::<u32>::new(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            UiLunexPlugins,
            //UiLunexDebugPlugin::<1, 2>,
        ))
        .add_systems(Startup, (ui_layout::setup, setup))
        .add_systems(
            Startup,
            (
                ui_behavior::setup.after(crate::ui_layout::setup),
                ui_behavior::setup_mouse_action.after(crate::ui_layout::setup),
            ),
        )
        .add_systems(
            Update,
            (ui_behavior::update, ui_behavior::mouse_action_cleanup),
        )
        .insert_resource(ui_behavior::UiState::new())
        .add_observer(ui_behavior::resolution_changed_observer)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[0, 1]),
    ));
}
