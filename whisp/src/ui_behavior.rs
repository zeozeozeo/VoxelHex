use crate::components::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_lunex::prelude::*;

#[derive(Resource)]
pub(crate) struct UiState {
    output_resolution_linked: bool,
    viewport_resolution_linked: bool,
}

impl UiState {
    pub(crate) fn new() -> Self {
        Self {
            output_resolution_linked: true,
            viewport_resolution_linked: true,
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
    mut output_width_button: Query<(Entity, &Output, &Width, &Button, &mut Text2d)>,
    mut output_height_button: Query<
        (Entity, &Output, &Height, &Button, &mut Text2d),
        Without<Width>,
    >,
    mut viewport_width_button: Query<
        (
            Entity,
            &crate::components::Camera,
            &Width,
            &Button,
            &mut Text2d,
        ),
        Without<Output>,
    >,
    mut viewport_height_button: Query<
        (
            Entity,
            &crate::components::Camera,
            &Height,
            &Button,
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
            height_text.0 = ((update.to as f32 * ratio) as i32).to_string();
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
            width_text.0 = ((update.to as f32 * ratio) as i32).to_string();
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
            height_text.0 = ((update.to as f32 * ratio) as i32).to_string();
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
            width_text.0 = ((update.to as f32 * ratio) as i32).to_string();
        }
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    output_resolution_linked: Query<(Entity, &Output, &Link, &Button)>,
    viewport_resolution_linked: Query<(Entity, &crate::components::Camera, &Link, &Button)>,
    info_panel_button_mini: Query<(Entity, &Info, &Button), Without<Expanded>>,
    info_panel_button_expanded: Query<(Entity, &Info, &Button), With<Expanded>>,
    fov_slider: Query<(Entity, &UiAction, &Slider, &crate::components::Camera)>,
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
            &Button,
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
            &Button,
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
        },
    );

    let (info_panel_button_expanded, _, _) = info_panel_button_expanded
        .single()
        .expect("Expected Close Shortcuts Panel Button to be available in UI");
    commands.entity(info_panel_button_expanded).observe(
        move |_: Trigger<Pointer<Click>>,
              mut commands: Commands,
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
        },
    );

    // FOV slider
    let (fov_slider, _, _, _) = fov_slider
        .single()
        .expect("Expected FOV Slider to be available in UI");
    commands.entity(fov_slider).observe(
        |mouse_move: Trigger<Pointer<Move>>,
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
            let (mut bar_size, mut bar_transform, _, _, _) = fov_slider_bar
                .single_mut()
                .expect("Expected FOV Slider bar to be available in UI");
            if ui_action.is_active {
                bar_size.x = mouse_move.hit.position.unwrap().x
                    - container_transform.translation().x
                    + container_size.x / 2.;
                bar_transform.translation.x = (bar_size.x - container_size.x) / 2.;
            }
        },
    );
}

pub(crate) fn setup_mouse_action(mut commands: Commands, action_query: Query<(Entity, &UiAction)>) {
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
}

fn update_number(mut number: i32, update_motion: &Vec2, ui_action: &UiAction) -> i32 {
    let update_count = update_motion.x - update_motion.y;
    number =
        (number as f32 * (1. + ui_action.change_sensitivity * (update_count as f32 / 4.))) as i32;
    number.clamp(ui_action.boundaries[0], ui_action.boundaries[1])
}

pub(crate) fn update(
    mut commands: Commands,
    ui_state: Res<UiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut motion: EventReader<MouseMotion>,
    mut output_resolution_width_update_button: Query<(
        &UiAction,
        &mut Text2d,
        &Output,
        &Width,
        &Button,
    )>,
    mut output_resolution_height_update_button: Query<
        (&UiAction, &mut Text2d, &Output, &Height, &Button),
        Without<Width>,
    >,
    mut viewport_resolution_width_update_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &crate::components::Camera,
            &Width,
            &Button,
        ),
        Without<Output>,
    >,
    mut viewport_resolution_height_update_button: Query<
        (
            &UiAction,
            &mut Text2d,
            &crate::components::Camera,
            &Height,
            &Button,
        ),
        (Without<Width>, Without<Output>),
    >,

    loading_panel: Query<(&Dimension, &Model, &Loading, &Slider), With<Container>>,
    mut loading_panel_bar: Query<
        (&mut Dimension, &mut Transform, &Model, &Loading, &Slider),
        Without<Container>,
    >,
) {
    // Get mouse update motion:
    let mouse_update_motion = motion.read().map(|ev| ev.delta).sum();

    // Viewport resolution Width update buttons
    for (ui_action, mut text, _, _, _) in viewport_resolution_width_update_button.iter_mut() {
        if ui_action.is_active {
            let width = text
                .0
                .parse::<i32>()
                .expect("Expected text to be parsable as number");
            let new_width = update_number(width, &mouse_update_motion, ui_action);
            text.0 = new_width.to_string();
            if ui_state.viewport_resolution_linked && width != new_width {
                commands.trigger(OutputResolutionUpdated {
                    by: ResolutionUpdated::ViewportWidth,
                    from: width,
                    to: new_width,
                });
            }
        }
    }

    // Viewport resolution Height update buttons
    for (ui_action, mut text, _, _, _) in viewport_resolution_height_update_button.iter_mut() {
        if ui_action.is_active {
            let height = text
                .0
                .parse::<i32>()
                .expect("Expected text to be parsable as number");
            let new_height = update_number(height, &mouse_update_motion, ui_action);
            text.0 = new_height.to_string();
            if ui_state.viewport_resolution_linked && height != new_height {
                commands.trigger(OutputResolutionUpdated {
                    by: ResolutionUpdated::ViewportHeight,
                    from: height,
                    to: new_height,
                });
            }
        }
    }
    // Output resolution Width update buttons
    for (ui_action, mut text, _, _, _) in output_resolution_width_update_button.iter_mut() {
        if ui_action.is_active {
            let width = text
                .0
                .parse::<i32>()
                .expect("Expected text to be parsable as number");
            let new_width = update_number(width, &mouse_update_motion, ui_action);
            text.0 = new_width.to_string();
            if ui_state.output_resolution_linked && width != new_width {
                commands.trigger(OutputResolutionUpdated {
                    by: ResolutionUpdated::OutputWidth,
                    from: width,
                    to: new_width,
                });
            }
        }
    }

    // Output resolution Height update buttons
    for (ui_action, mut text, _, _, _) in output_resolution_height_update_button.iter_mut() {
        if ui_action.is_active {
            let height = text
                .0
                .parse::<i32>()
                .expect("Expected text to be parsable as number");
            let new_height = update_number(height, &mouse_update_motion, ui_action);
            text.0 = new_height.to_string();
            if ui_state.output_resolution_linked && height != new_height {
                commands.trigger(OutputResolutionUpdated {
                    by: ResolutionUpdated::OutputHeight,
                    from: height,
                    to: new_height,
                });
            }
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
