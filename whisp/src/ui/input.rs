use crate::{ui::components::*, ui::UiState};
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_pkv::PkvStore;

pub(crate) fn setup_mouse_action(
    mut commands: Commands,
    action_query: Query<(Entity, &UiAction)>,
    fov_slider_bar: Query<
        (Entity, &crate::ui::components::Camera, &Depth, &Slider),
        Without<Container>,
    >,
) {
    for (entity, _) in action_query.iter() {
        commands.entity(entity).observe(
            |trigger: Trigger<Pointer<Pressed>>, action_query: Query<(Entity, &mut UiAction)>| {
                for (entity, mut ui_action) in action_query {
                    if entity == trigger.target {
                        ui_action.is_active = true;
                    }
                }
            },
        );
    }

    let (fov_slider_bar, _, _, _) = fov_slider_bar
        .single()
        .expect("Expected FOV Slider bar to be available in UI");
    commands.entity(fov_slider_bar).observe(
        |_: Trigger<Pointer<Pressed>>,
         mut fov_slider: Query<(
            &mut UiAction,
            &crate::ui::components::Camera,
            &Depth,
            &Slider,
            &Container,
        )>| {
            let (mut ui_action, _, _, _, _) = fov_slider
                .single_mut()
                .expect("Expected FOV Slider to be available in UI");
            ui_action.is_active = true;
        },
    );
}

pub(crate) fn mouse_action_cleanup(
    buttons: Res<ButtonInput<MouseButton>>,
    mut ui_action_items_query: Query<&mut UiAction>,
) {
    if buttons.just_released(MouseButton::Left) {
        for mut item in ui_action_items_query.iter_mut() {
            item.is_active = false;
        }
    }
}

pub(crate) fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
    mut ui_state: ResMut<UiState>,
    mut ui_container: Query<(&mut Visibility, &UserInterface)>,
    mut info_panel_button_mini: Query<
        (&mut Visibility, &Info, &Container),
        (Without<Expanded>, Without<UserInterface>),
    >,
    mut info_panel_button_expanded: Query<
        (&mut Visibility, &Info, &Container),
        (With<Expanded>, Without<UserInterface>),
    >,
    mut output_width_button: Query<(&mut Text2d, &Output, &Width, &crate::ui::components::Button)>,
    mut output_height_button: Query<
        (
            &mut Text2d,
            &Output,
            &Height,
            &crate::ui::components::Button,
        ),
        Without<Width>,
    >,
    mut viewport_width_button: Query<
        (
            &mut Text2d,
            &crate::ui::components::Camera,
            &Width,
            &crate::ui::components::Button,
        ),
        Without<Output>,
    >,
    mut viewport_height_button: Query<
        (
            &mut Text2d,
            &crate::ui::components::Camera,
            &Height,
            &crate::ui::components::Button,
        ),
        (Without<Width>, Without<Output>),
    >,
    fov_slider: Query<(
        &UiAction,
        &Dimension,
        &crate::ui::components::Camera,
        &Depth,
        &Slider,
        &Container,
    )>,
    mut fov_slider_bar: Query<
        (
            &mut Dimension,
            &mut Transform,
            &crate::ui::components::Camera,
            &Depth,
            &Slider,
        ),
        Without<Container>,
    >,
    mut view_distance_button: Query<
        (
            &mut Text2d,
            &crate::ui::components::Camera,
            &Depth,
            &crate::ui::components::Button,
        ),
        (Without<Width>, Without<Height>, Without<Output>),
    >,
    mut output_resolution_linked: Query<(
        &mut Sprite,
        &Output,
        &Link,
        &crate::ui::components::Button,
    )>,
    mut viewport_resolution_linked: Query<
        (
            &mut Sprite,
            &crate::ui::components::Camera,
            &Link,
            &crate::ui::components::Button,
        ),
        (
            Without<Width>,
            Without<Height>,
            Without<Output>,
            Without<Depth>,
        ),
    >,
) {
    // Hiding UI
    if keys.just_pressed(KeyCode::KeyG) {
        let (mut visibility, _) = ui_container
            .single_mut()
            .expect("Expected UI to be available");
        let (mut mini_visibility, _, _) = info_panel_button_mini
            .single_mut()
            .expect("Expected Open Shortcuts Panel Button to be available in UI");
        let (mut expanded_visibility, _, _) = info_panel_button_expanded
            .single_mut()
            .expect("Expected Close Shortcuts Panel Button to be available in UI");
        ui_state.hide_ui = !ui_state.hide_ui;
        pkv.set("ui_hidden", &ui_state.hide_ui.to_string())
            .expect("Expected to be able to store setting ui_hidden!");
        if ui_state.hide_ui {
            *visibility = Visibility::Hidden;
            *mini_visibility = Visibility::Hidden;
            *expanded_visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
            if ui_state.hide_shortcuts {
                *mini_visibility = Visibility::Visible;
                *expanded_visibility = Visibility::Hidden;
            } else {
                *mini_visibility = Visibility::Hidden;
                *expanded_visibility = Visibility::Visible;
            }
        }
    }

    let (mut output_width_text, _, _, _) = output_width_button
        .single_mut()
        .expect("Expected Output Width Button to be available in UI");

    let (mut output_height_text, _, _, _) = output_height_button
        .single_mut()
        .expect("Expected Output Height Button to be available in UI");

    let (mut viewport_width_text, _, _, _) = viewport_width_button
        .single_mut()
        .expect("Expected Viewport Width Button to be available in UI");

    let (mut viewport_height_text, _, _, _) = viewport_height_button
        .single_mut()
        .expect("Expected Viewport Height Button to be available in UI");

    let (mut view_distance_text, _, _, _) = view_distance_button
        .single_mut()
        .expect("Expected View distance Button to be available in UI");

    let (fov_ui_action, fov_container_size, _, _, _, _) = fov_slider
        .single()
        .expect("Expected FOV Slider to be available in UI");
    let (mut fov_bar_size, mut fov_bar_transform, _, _, _) = fov_slider_bar
        .single_mut()
        .expect("Expected FOV Slider bar to be available in UI");

    let (mut output_resolution_sprite, _, _, _) = output_resolution_linked
        .single_mut()
        .expect("Expected Output Resolution Linked Button to be available in UI");

    let (mut viewport_resolution_sprite, _, _, _) = viewport_resolution_linked
        .single_mut()
        .expect("Expected Output Resolution Linked Button to be available in UI");

    let fov_bar_value_extent = (fov_ui_action.boundaries[1] - fov_ui_action.boundaries[0]) as f32;
    let fov_bar_unit = fov_container_size.x / fov_bar_value_extent;

    // Saving settings: output_resolution, viewport_resolution, fov, view_distance
    if keys.just_pressed(KeyCode::F5) {
        pkv.set("output_resolution_width", &output_width_text.0)
            .expect("Failed to store value: output_resolution_width");
        pkv.set("output_resolution_height", &output_height_text.0)
            .expect("Failed to store value: output_resolution_height");
        pkv.set("viewport_resolution_width", &viewport_width_text.0)
            .expect("Failed to store value: viewport_resolution_width");
        pkv.set("viewport_resolution_height", &viewport_height_text.0)
            .expect("Failed to store value: output_resolution_height");
        pkv.set("fov", &ui_state.fov_value.to_string())
            .expect("Failed to store value: fov");
        pkv.set("view_distance", &ui_state.view_distance.to_string())
            .expect("Failed to store value: view_distance");
        pkv.set(
            "output_resolution_linked",
            &ui_state.output_resolution_linked.to_string(),
        )
        .expect("Expected to be able to store setting output_resolution_linked!");
        pkv.set(
            "viewport_resolution_linked",
            &ui_state.viewport_resolution_linked.to_string(),
        )
        .expect("Expected to be able to store setting viewport_resolution_linked!");
    }

    // Loading settings
    if keys.just_pressed(KeyCode::F9) {
        ui_state.fov_value = pkv
            .get::<String>("fov")
            .ok()
            .unwrap_or_else(|| "50".to_string())
            .parse()
            .expect("Expected fov setting to be a parsable number");
        output_width_text.0 = pkv
            .get::<String>("output_resolution_width")
            .ok()
            .unwrap_or_else(|| "1920".to_string());
        output_height_text.0 = pkv
            .get::<String>("output_resolution_height")
            .ok()
            .unwrap_or_else(|| "1080".to_string());
        viewport_width_text.0 = pkv
            .get::<String>("viewport_resolution_width")
            .ok()
            .unwrap_or_else(|| "100".to_string());
        viewport_height_text.0 = pkv
            .get::<String>("viewport_resolution_height")
            .ok()
            .unwrap_or_else(|| "100".to_string());
        view_distance_text.0 = pkv
            .get::<String>("view_distance")
            .ok()
            .unwrap_or_else(|| "1024".to_string());

        let fov_bar_percentage =
            (ui_state.fov_value - fov_ui_action.boundaries[0]) as f32 / fov_bar_value_extent;
        fov_bar_size.x = fov_bar_unit * fov_bar_percentage * 100.;
        fov_bar_transform.translation.x = (fov_bar_size.x - fov_container_size.x) / 2.;

        ui_state.output_resolution = [
            output_width_text
                .0
                .parse::<u32>()
                .expect("Expected output_resolution_width setting to be a parsable number"),
            output_height_text
                .0
                .parse::<u32>()
                .expect("Expected output_resolution_height setting to be a parsable number"),
        ];

        ui_state.viewport_resolution = [
            viewport_width_text
                .0
                .parse::<u32>()
                .expect("Expected viewport_resolution_width setting to be a parsable number"),
            viewport_height_text
                .0
                .parse::<u32>()
                .expect("Expected viewport_resolution_width setting to be a parsable number"),
        ];

        ui_state.view_distance = view_distance_text
            .0
            .parse::<u32>()
            .expect("Expected view distance setting to be a parsable number");

        ui_state.output_resolution_linked = if let Ok(link) =
            pkv.get::<String>("output_resolution_linked")
        {
            link.parse::<bool>()
                .expect("Expected output_resolution_linked setting to be either 'true' or 'false'")
        } else {
            true
        };
        ui_state.viewport_resolution_linked =
            if let Ok(link) = pkv.get::<String>("viewport_resolution_linked") {
                link.parse::<bool>().expect(
                    "Expected viewport_resolution_linked setting to be either 'true' or 'false'",
                )
            } else {
                true
            };
    }

    // Restoring default settings
    if keys.just_pressed(KeyCode::F11) {
        pkv.set("output_resolution_width", &"1920")
            .expect("Failed to store value: output_resolution_width");
        pkv.set("output_resolution_height", &"1080")
            .expect("Failed to store value: output_resolution_height");
        pkv.set("viewport_resolution_width", &"100")
            .expect("Failed to store value: viewport_resolution_width");
        pkv.set("viewport_resolution_height", &"100")
            .expect("Failed to store value: output_resolution_height");
        pkv.set("fov", &"50").expect("Failed to store value: fov");
        pkv.set("view_distance", &"1024")
            .expect("Failed to store value: view_distance");

        ui_state.output_resolution = [1920, 1080];
        ui_state.viewport_resolution = [100, 100];
        ui_state.view_distance = 1024;
        ui_state.fov_value = 50;
        ui_state.output_resolution_linked = true;
        ui_state.viewport_resolution_linked = true;

        fov_bar_size.x = fov_bar_unit * 50.;
        fov_bar_transform.translation.x = (fov_bar_size.x - fov_container_size.x) / 2.;
        output_width_text.0 = "1920".to_string();
        output_height_text.0 = "1080".to_string();
        viewport_width_text.0 = "100".to_string();
        viewport_height_text.0 = "100".to_string();
        view_distance_text.0 = "1024".to_string();

        *output_resolution_sprite = Sprite::from_image(asset_server.load("ui/linked_icon.png"));
        *viewport_resolution_sprite = Sprite::from_image(asset_server.load("ui/linked_icon.png"));
    }
}
