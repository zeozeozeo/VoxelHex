mod ui;

use bevy::{prelude::*, render::view::RenderLayers, window::WindowPlugin};
use bevy_lunex::prelude::*;
use bevy_pkv::PkvStore;

fn main() {
    let preferences = init_preferences_cache();
    let ui_state = ui::UiState::new(&preferences);
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
        .add_systems(Startup, (ui::layout::setup, setup))
        .add_systems(
            Startup,
            ((
                ui::input::setup_mouse_action,
                ui::behavior::setup,
                ui::behavior::messages,
            )
                .after(crate::ui::layout::setup),),
        )
        .add_systems(
            Update,
            (
                ui::behavior::update,
                ui::input::mouse_action_cleanup,
                ui::input::keyboard_input,
            ),
        )
        .insert_resource(ui_state)
        .insert_resource(preferences)
        .add_observer(ui::behavior::resolution_changed_observer)
        .run();
}

fn init_preferences_cache() -> PkvStore {
    let mut pkv = PkvStore::new("MinistryOfVoxelAffairs", "Whisp");
    if pkv.get::<String>("output_resolution_width").is_err() {
        pkv.set("output_resolution_width", &"1920")
            .expect("Failed to store default value: output_resolution_width");
    }
    if pkv.get::<String>("output_resolution_height").is_err() {
        pkv.set("output_resolution_height", &"1080")
            .expect("Failed to store default value: output_resolution_height");
    }
    if pkv.get::<String>("fov").is_err() {
        pkv.set("fov", &"50")
            .expect("Failed to store default value: fov");
    }
    if pkv.get::<String>("viewport_resolution_width").is_err() {
        pkv.set("viewport_resolution_width", &"100")
            .expect("Failed to store default value: viewport_resolution_width");
    }
    if pkv.get::<String>("viewport_resolution_height").is_err() {
        pkv.set("viewport_resolution_height", &"100")
            .expect("Failed to store default value: viewport_resolution_height");
    }
    if pkv.get::<String>("view_distance").is_err() {
        pkv.set("view_distance", &"1024")
            .expect("Failed to store default value: view_distance");
    }
    if pkv.get::<String>("ui_hidden").is_err() {
        pkv.set("ui_hidden", &"false")
            .expect("Failed to store default value: ui_hidden");
    }
    if pkv.get::<String>("shortcuts_hidden").is_err() {
        pkv.set("shortcuts_hidden", &"false")
            .expect("Failed to store default value: shortcuts_hidden");
    }
    if pkv.get::<String>("output_resolution_linked").is_err() {
        pkv.set("output_resolution_linked", &"false")
            .expect("Failed to store default value: output_resolution_linked");
    }
    if pkv.get::<String>("viewport_resolution_linked").is_err() {
        pkv.set("viewport_resolution_linked", &"false")
            .expect("Failed to store default value: viewport_resolution_linked");
    }
    pkv
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[0, 1]),
    ));
}
