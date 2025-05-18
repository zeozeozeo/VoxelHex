use crate::components::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_lunex::prelude::*;
use bevy_pkv::PkvStore;

#[derive(Resource)]
pub(crate) struct UiState {
    hide_ui: bool,
    hide_shortcuts: bool,
    output_resolution_linked: bool,
    viewport_resolution_linked: bool,
    fov_value: i32,
    view_distance: i32,
    output_resolution: [i32; 2],
    viewport_resolution: [i32; 2],
}

impl UiState {
    pub(crate) fn new(pkv: &PkvStore) -> Self {
        Self {
            hide_ui: if let Ok(link) = pkv.get::<String>("ui_hidden") {
                link.parse::<bool>()
                    .expect("Expected ui_hidden setting to be either 'true' or 'false'")
            } else {
                true
            },
            hide_shortcuts: if let Ok(link) = pkv.get::<String>("shortcuts_hidden") {
                link.parse::<bool>()
                    .expect("Expected stored shortcuts_hidden to be either 'true' or 'false'")
            } else {
                true
            },
            output_resolution_linked: if let Ok(link) =
                pkv.get::<String>("output_resolution_linked")
            {
                link.parse::<bool>().expect(
                    "Expected output_resolution_linked setting to be either 'true' or 'false'",
                )
            } else {
                true
            },
            viewport_resolution_linked: if let Ok(link) =
                pkv.get::<String>("viewport_resolution_linked")
            {
                link.parse::<bool>().expect(
                    "Expected viewport_resolution_linked setting to be either 'true' or 'false'",
                )
            } else {
                true
            },
            fov_value: if let Ok(fov) = pkv.get::<String>("fov") {
                fov.parse::<i32>()
                    .expect("Expected fov setting to be a parsable number")
            } else {
                50
            },
            view_distance: if let Ok(vdist) = pkv.get::<String>("view_distance") {
                vdist
                    .parse::<i32>()
                    .expect("Expected view_distance setting to be a parsable number")
            } else {
                1024
            },
            output_resolution: [
                if let Ok(res) = pkv.get::<String>("output_resolution_width") {
                    res.parse::<i32>()
                        .expect("Expected output_resolution_width setting to be a parsable number")
                } else {
                    1920
                },
                if let Ok(res) = pkv.get::<String>("output_resolution_height") {
                    res.parse::<i32>()
                        .expect("Expected output_resolution_height setting to be a parsable number")
                } else {
                    1080
                },
            ],
            viewport_resolution: [
                if let Ok(res) = pkv.get::<String>("viewport_resolution_width") {
                    res.parse::<i32>().expect(
                        "Expected viewport_resolution_width setting to be a parsable number",
                    )
                } else {
                    100
                },
                if let Ok(res) = pkv.get::<String>("viewport_resolution_height") {
                    res.parse::<i32>().expect(
                        "Expected viewport_resolution_height setting to be a parsable number",
                    )
                } else {
                    100
                },
            ],
        }
    }
}

enum ResolutionUpdated {
    OutputWidth,
    OutputHeight,
    ViewportWidth,
    ViewportHeight,
}

#[derive(Event)]
#[event(auto_propagate)]
pub(crate) struct OutputResolutionUpdated {
    by: ResolutionUpdated,
    from: i32,
    to: i32,
}

pub(crate) fn resolution_changed_observer(
    update: Trigger<OutputResolutionUpdated>,
    mut ui_state: ResMut<UiState>,
    mut output_width_button: Query<(
        Entity,
        &Output,
        &Width,
        &crate::components::Button,
        &mut Text2d,
    )>,
    mut output_height_button: Query<
        (
            Entity,
            &Output,
            &Height,
            &crate::components::Button,
            &mut Text2d,
        ),
        Without<Width>,
    >,
    mut viewport_width_button: Query<
        (
            Entity,
            &crate::components::Camera,
            &Width,
            &crate::components::Button,
            &mut Text2d,
        ),
        Without<Output>,
    >,
    mut viewport_height_button: Query<
        (
            Entity,
            &crate::components::Camera,
            &Height,
            &crate::components::Button,
            &mut Text2d,
        ),
        (Without<Width>, Without<Output>),
    >,
) {
    match update.by {
        ResolutionUpdated::OutputWidth => {
            let (_, _, _, _, mut height_text) = output_height_button
                .single_mut()
                .expect("Expected Output Height Button to be available in UI");
            let ratio = height_text
                .0
                .parse::<i32>()
                .expect("Expected Output Height to be a parsable integer!")
                as f32
                / update.from as f32;
            let new_height = (update.to as f32 * ratio) as i32;
            height_text.0 = new_height.to_string();
            ui_state.output_resolution[1] = new_height;
        }
        ResolutionUpdated::OutputHeight => {
            let (_, _, _, _, mut width_text) = output_width_button
                .single_mut()
                .expect("Expected Output Width Button to be available in UI");
            let ratio = width_text
                .0
                .parse::<i32>()
                .expect("Expected Output Width to be a parsable integer!")
                as f32
                / update.from as f32;
            let new_width = (update.to as f32 * ratio) as i32;
            width_text.0 = new_width.to_string();
            ui_state.output_resolution[0] = new_width;
        }
        ResolutionUpdated::ViewportWidth => {
            let (_, _, _, _, mut height_text) = viewport_height_button
                .single_mut()
                .expect("Expected Viewport Height Button to be available in UI");
            let ratio = height_text
                .0
                .parse::<i32>()
                .expect("Expected Viewport Height to be a parsable integer!")
                as f32
                / update.from as f32;
            let new_height = (update.to as f32 * ratio) as i32;
            height_text.0 = new_height.to_string();
            ui_state.viewport_resolution[1] = new_height;
        }
        ResolutionUpdated::ViewportHeight => {
            let (_, _, _, _, mut width_text) = viewport_width_button
                .single_mut()
                .expect("Expected Viewport Width Button to be available in UI");
            let ratio = width_text
                .0
                .parse::<i32>()
                .expect("Expected Viewport Height to be a parsable integer!")
                as f32
                / update.from as f32;
            let new_width = (update.to as f32 * ratio) as i32;
            width_text.0 = new_width.to_string();
            ui_state.viewport_resolution[0] = new_width;
        }
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    output_resolution_linked: Query<(Entity, &Output, &Link, &crate::components::Button)>,
    viewport_resolution_linked: Query<(
        Entity,
        &crate::components::Camera,
        &Link,
        &crate::components::Button,
    )>,
    info_panel_button_mini: Query<(Entity, &Info, &crate::components::Button), Without<Expanded>>,
    info_panel_button_expanded: Query<(Entity, &Info, &crate::components::Button), With<Expanded>>,
    fov_slider: Query<(
        Entity,
        &UiAction,
        &crate::components::Camera,
        &Depth,
        &Slider,
        &Container,
    )>,
    fov_slider_bar: Query<
        (Entity, &crate::components::Camera, &Depth, &Slider),
        Without<Container>,
    >,
) {
    // Viewport resolution linked
    let (viewport_resolution_link_button, _, _, _) = viewport_resolution_linked
        .single()
        .expect("Expected Output Resolution Linked Button to be available in UI");
    commands.entity(viewport_resolution_link_button).observe(
        move |_: Trigger<Pointer<Click>>,
              mut ui_state: ResMut<UiState>,
              asset_server: Res<AssetServer>,
              mut viewport_resolution_linked: Query<(
            &mut Sprite,
            &crate::components::Camera,
            &Link,
            &crate::components::Button,
        )>| {
            let (mut link_sprite, _, _, _) = viewport_resolution_linked
                .single_mut()
                .expect("Expected Output Resolution Linked Button to be available in UI");

            ui_state.viewport_resolution_linked = !ui_state.viewport_resolution_linked;
            *link_sprite = if ui_state.viewport_resolution_linked {
                Sprite::from_image(asset_server.load("ui/linked_icon.png"))
            } else {
                Sprite::from_image(asset_server.load("ui/not_linked_icon.png"))
            };
        },
    );

    // Output resolution linked
    let (output_resolution_link_button, _, _, _) = output_resolution_linked
        .single()
        .expect("Expected Output Resolution Linked Button to be available in UI");
    commands.entity(output_resolution_link_button).observe(
        move |_: Trigger<Pointer<Click>>,
              mut ui_state: ResMut<UiState>,
              asset_server: Res<AssetServer>,
              mut output_resolution_linked: Query<(
            &mut Sprite,
            &crate::components::Output,
            &Link,
            &crate::components::Button,
        )>| {
            let (mut link_sprite, _, _, _) = output_resolution_linked
                .single_mut()
                .expect("Expected Output Resolution Linked Button to be available in UI");

            ui_state.output_resolution_linked = !ui_state.output_resolution_linked;
            *link_sprite = if ui_state.output_resolution_linked {
                Sprite::from_image(asset_server.load("ui/linked_icon.png"))
            } else {
                Sprite::from_image(asset_server.load("ui/not_linked_icon.png"))
            };
        },
    );

    // Shortcuts panel
    let (info_panel_button_mini, _, _) = info_panel_button_mini
        .single()
        .expect("Expected Open Shortcuts Panel Button to be available in UI");
    commands.entity(info_panel_button_mini).observe(
        move |_: Trigger<Pointer<Click>>,
              mut commands: Commands,
              mut pkv: ResMut<PkvStore>,
              mut ui_state: ResMut<UiState>,
              info_panel_mini: Query<(Entity, &Info, &Container), Without<Expanded>>,
              info_panel_expaned: Query<(Entity, &Info, &Container), With<Expanded>>| {
            let (maxi_info_panel, _, _) = info_panel_expaned
                .single()
                .expect("Expected Expanded Shortcuts Panel to be available in UI");
            let (mini_info_panel, _, _) = info_panel_mini
                .single()
                .expect("Expected Mini Shortcuts Button to be available in UI");
            commands.entity(maxi_info_panel).insert(Visibility::Visible);
            commands.entity(mini_info_panel).insert(Visibility::Hidden);
            ui_state.hide_shortcuts = false;
            pkv.set("shortcuts_hidden", &ui_state.hide_shortcuts.to_string())
                .expect("Expected to be able to store setting shortcuts_hidden");
        },
    );
    let (info_panel_button_expanded, _, _) = info_panel_button_expanded
        .single()
        .expect("Expected Close Shortcuts Panel Button to be available in UI");
    commands.entity(info_panel_button_expanded).observe(
        move |_: Trigger<Pointer<Click>>,
              mut commands: Commands,
              mut pkv: ResMut<PkvStore>,
              mut ui_state: ResMut<UiState>,
              info_panel_mini: Query<(Entity, &Info, &Container), Without<Expanded>>,
              info_panel_expaned: Query<(Entity, &Info, &Container), With<Expanded>>| {
            let (maxi_info_panel, _, _) = info_panel_expaned
                .single()
                .expect("Expected Expanded Shortcuts Panel to be available in UI");
            let (mini_info_panel, _, _) = info_panel_mini
                .single()
                .expect("Expected Mini Shortcuts Button to be available in UI");
            commands.entity(maxi_info_panel).insert(Visibility::Hidden);
            commands.entity(mini_info_panel).insert(Visibility::Visible);
            ui_state.hide_shortcuts = true;
            pkv.set("shortcuts_hidden", &ui_state.hide_shortcuts.to_string())
                .expect("Expected to be able to store setting shortcuts_hidden");
        },
    );

    // FOV slider
    let fov_slider_observer = |mouse_move: Trigger<Pointer<Move>>,
                               mut ui_state: ResMut<UiState>,
                               fov_slider: Query<(
        &UiAction,
        &Dimension,
        &GlobalTransform,
        &crate::components::Camera,
        &Depth,
        &Slider,
        &Container,
    )>,
                               mut fov_slider_bar: Query<
        (
            &mut Dimension,
            &mut Transform,
            &crate::components::Camera,
            &Depth,
            &Slider,
        ),
        Without<Container>,
    >| {
        let (ui_action, container_size, container_transform, _, _, _, _) = fov_slider
            .single()
            .expect("Expected FOV Slider to be available in UI");
        if ui_action.is_active {
            let (mut bar_size, mut bar_transform, _, _, _) = fov_slider_bar
                .single_mut()
                .expect("Expected FOV Slider bar to be available in UI");
            bar_size.x = mouse_move.hit.position.unwrap().x - container_transform.translation().x
                + container_size.x / 2.;
            bar_transform.translation.x = (bar_size.x - container_size.x) / 2.;
            let fov_bar_value_extent = (ui_action.boundaries[1] - ui_action.boundaries[0]) as f32;
            let fov_bar_percentage = (bar_size.x / container_size.x) as f32;
            ui_state.fov_value =
                (ui_action.boundaries[0] as f32 + fov_bar_value_extent * fov_bar_percentage) as i32;
        }
    };

    let (fov_slider, _, _, _, _, _) = fov_slider
        .single()
        .expect("Expected FOV Slider to be available in UI");

    let (fov_slider_bar, _, _, _) = fov_slider_bar
        .single()
        .expect("Expected FOV Slider bar to be available in UI");

    commands.entity(fov_slider).observe(fov_slider_observer);
    commands.entity(fov_slider_bar).observe(fov_slider_observer);
}

pub(crate) fn setup_mouse_action(
    mut commands: Commands,
    action_query: Query<(Entity, &UiAction)>,
    fov_slider_bar: Query<
        (Entity, &crate::components::Camera, &Depth, &Slider),
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
            &crate::components::Camera,
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

fn update_number(number: i32, update_motion: &Vec2, ui_action: &UiAction) -> i32 {
    let update_count = update_motion.x - update_motion.y;
    let mut new_number =
        (number as f32 * (1. + ui_action.change_sensitivity * (update_count as f32 / 4.))) as i32;
    if new_number == number && 0. != update_count {
        new_number += update_count.signum() as i32;
    }
    new_number.clamp(ui_action.boundaries[0], ui_action.boundaries[1])
}

pub(crate) fn update(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut motion: EventReader<MouseMotion>,
    mut output_resolution_width_update_button: Query<(
        &UiAction,
        &mut Text2d,
        &Output,
        &Width,
        &crate::components::Button,
    )>,
    mut output_resolution_height_update_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &Output,
            &Height,
            &crate::components::Button,
        ),
        Without<Width>,
    >,
    mut viewport_resolution_width_update_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &crate::components::Camera,
            &Width,
            &crate::components::Button,
        ),
        Without<Output>,
    >,
    mut viewport_resolution_height_update_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &crate::components::Camera,
            &Height,
            &crate::components::Button,
        ),
        (Without<Width>, Without<Output>),
    >,
    mut view_distance_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &crate::components::Camera,
            &Depth,
            &crate::components::Button,
        ),
        (Without<Width>, Without<Height>, Without<Output>),
    >,
    loading_panel: Query<(&Dimension, &Model, &Loading, &Slider), With<Container>>,
    mut loading_panel_bar: Query<
        (&mut Dimension, &mut Transform, &Model, &Loading, &Slider),
        Without<Container>,
    >,
) {
    // Get mouse update motion:
    let mouse_update_motion = motion.read().map(|ev| ev.delta).sum();

    // View distance button
    let (ui_action, mut text, _, _, _) = view_distance_button
        .single_mut()
        .expect("Expected View distance Button to be available in UI");
    if ui_action.is_active {
        let distance = text
            .0
            .parse::<i32>()
            .expect("Expected viewport width text to be parsable as number");
        let new_distance = update_number(distance, &mouse_update_motion, ui_action);
        text.0 = new_distance.to_string();
        ui_state.view_distance = new_distance;
    }

    // Viewport resolution Width update button
    let (ui_action, mut text, _, _, _) = viewport_resolution_width_update_button
        .single_mut()
        .expect("Expected Viewport Resolution Width Button to be available in UI");
    if ui_action.is_active {
        let width = text
            .0
            .parse::<i32>()
            .expect("Expected viewport width text to be parsable as number");
        let new_width = update_number(width, &mouse_update_motion, ui_action);
        text.0 = new_width.to_string();
        ui_state.output_resolution[0] = new_width;
        if ui_state.viewport_resolution_linked && width != new_width {
            commands.trigger(OutputResolutionUpdated {
                by: ResolutionUpdated::ViewportWidth,
                from: width,
                to: new_width,
            });
        }
    }

    // Viewport resolution Height update button
    let (ui_action, mut text, _, _, _) = viewport_resolution_height_update_button
        .single_mut()
        .expect("Expected Viewport Resolution Height Button to be available in UI");
    if ui_action.is_active {
        let height = text
            .0
            .parse::<i32>()
            .expect("Expected viewport height text to be parsable as number");
        let new_height = update_number(height, &mouse_update_motion, ui_action);
        text.0 = new_height.to_string();
        ui_state.output_resolution[1] = new_height;
        if ui_state.viewport_resolution_linked && height != new_height {
            commands.trigger(OutputResolutionUpdated {
                by: ResolutionUpdated::ViewportHeight,
                from: height,
                to: new_height,
            });
        }
    }

    // Output resolution Width update button
    let (ui_action, mut text, _, _, _) = output_resolution_width_update_button
        .single_mut()
        .expect("Expected Output Resolution Width Button to be available in UI");
    if ui_action.is_active {
        let width = text
            .0
            .parse::<i32>()
            .expect("Expected text to be parsable as number");
        let new_width = update_number(width, &mouse_update_motion, ui_action);
        ui_state.viewport_resolution[0] = new_width;

        text.0 = new_width.to_string();
        if ui_state.output_resolution_linked && width != new_width {
            commands.trigger(OutputResolutionUpdated {
                by: ResolutionUpdated::OutputWidth,
                from: width,
                to: new_width,
            });
        }
    }

    // Output resolution Height update button
    let (ui_action, mut text, _, _, _) = output_resolution_height_update_button
        .single_mut()
        .expect("Expected Output Resolution Height Button to be available in UI");
    if ui_action.is_active {
        let height = text
            .0
            .parse::<i32>()
            .expect("Expected text to be parsable as number");
        let new_height = update_number(height, &mouse_update_motion, ui_action);
        text.0 = new_height.to_string();
        ui_state.viewport_resolution[1] = new_height;

        if ui_state.output_resolution_linked && height != new_height {
            commands.trigger(OutputResolutionUpdated {
                by: ResolutionUpdated::OutputHeight,
                from: height,
                to: new_height,
            });
        }
    }

    // Progress bar
    let (loading_panel_size, _, _, _) = loading_panel
        .single()
        .expect("Expected Model Progress panel to be available in UI");
    let (mut progressbar_size, mut progressbar_transform, _, _, _) = loading_panel_bar
        .single_mut()
        .expect("Expected Model Progressbar to be available in UI");

    let progressbar_xtrinsics_fn = |progress: f32, container_size: &Dimension| -> (f32, f32) {
        let size_x = container_size.x * progress;
        let transform_x = (size_x - container_size.x) / 2.;
        (size_x, transform_x)
    };
    if keys.just_pressed(KeyCode::Digit0) {
        (progressbar_size.x, progressbar_transform.translation.x) =
            progressbar_xtrinsics_fn(0., loading_panel_size);
    }
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
    mut output_width_button: Query<(&mut Text2d, &Output, &Width, &crate::components::Button)>,
    mut output_height_button: Query<
        (&mut Text2d, &Output, &Height, &crate::components::Button),
        Without<Width>,
    >,
    mut viewport_width_button: Query<
        (
            &mut Text2d,
            &crate::components::Camera,
            &Width,
            &crate::components::Button,
        ),
        Without<Output>,
    >,
    mut viewport_height_button: Query<
        (
            &mut Text2d,
            &crate::components::Camera,
            &Height,
            &crate::components::Button,
        ),
        (Without<Width>, Without<Output>),
    >,
    fov_slider: Query<(
        &UiAction,
        &Dimension,
        &crate::components::Camera,
        &Depth,
        &Slider,
        &Container,
    )>,
    mut fov_slider_bar: Query<
        (
            &mut Dimension,
            &mut Transform,
            &crate::components::Camera,
            &Depth,
            &Slider,
        ),
        Without<Container>,
    >,
    mut view_distance_button: Query<
        (
            &mut Text2d,
            &crate::components::Camera,
            &Depth,
            &crate::components::Button,
        ),
        (Without<Width>, Without<Height>, Without<Output>),
    >,
    mut output_resolution_linked: Query<(&mut Sprite, &Output, &Link, &crate::components::Button)>,
    mut viewport_resolution_linked: Query<
        (
            &mut Sprite,
            &crate::components::Camera,
            &Link,
            &crate::components::Button,
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
                .parse::<i32>()
                .expect("Expected output_resolution_width setting to be a parsable number"),
            output_height_text
                .0
                .parse::<i32>()
                .expect("Expected output_resolution_height setting to be a parsable number"),
        ];

        ui_state.viewport_resolution = [
            viewport_width_text
                .0
                .parse::<i32>()
                .expect("Expected viewport_resolution_width setting to be a parsable number"),
            viewport_height_text
                .0
                .parse::<i32>()
                .expect("Expected viewport_resolution_width setting to be a parsable number"),
        ];

        ui_state.view_distance = view_distance_text
            .0
            .parse::<i32>()
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

pub(crate) fn messages(mut status_text: Query<(&mut Text2d, &Model, &Status)>) {
    #[cfg(debug_assertions)]
    {
        let (mut message_text, _, _) = status_text
            .single_mut()
            .expect("Expected Status message to be available in UI");
        message_text.0 = "WARNING! Running in Debug mode! Perfromance will be bad!".to_string();
    }
}
