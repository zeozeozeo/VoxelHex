use crate::ui::{components::*, UiState};
use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use bevy_lunex::UiColor;
use bevy_pkv::PkvStore;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use voxelhex::{
    boxtree::{BoxTree, V3c},
    raytracing::{BoxTreeGPUHost, VhxViewSet},
};

const BRICK_DIMENSION: u32 = 32;

#[derive(Default)]
enum TreePayload {
    #[default]
    Unknown,
    Loading(Task<Result<BoxTree<u32>, String>>),
    Loaded(BoxTree<u32>),
}

#[derive(Resource)]
pub(crate) struct TreeLoadingTask {
    confirmed: bool,
    model_name: String,
    tmp_file_path: String,
    target_cache_file_path: String,
    payload: TreePayload,
}

fn delete_by_path<P: AsRef<Path>>(path: P) {
    if path.as_ref().exists() {
        std::fs::remove_file(path.as_ref()).expect(&format!(
            "Expected to be able to remove temporary file at ::{:?}::",
            path.as_ref()
                .to_str()
                .unwrap_or_else(|| "path_file_conversion_failed")
        ));
    }
}

fn load_task_from_path(path: &PathBuf, confirmed: bool) -> TreeLoadingTask {
    let tree_file_name = path
        .file_stem()
        .unwrap_or_else(|| OsStr::new("unknwon"))
        .to_str()
        .unwrap_or_else(|| "name_conversion_failed");
    let model_path = path
        .to_str()
        .unwrap_or_else(|| "name_conversion_failed")
        .to_string();
    let tmp_file_path_ = ".tmp_cache_".to_string() + tree_file_name;
    let tmp_file_path = tmp_file_path_.to_string();
    let target_cache_file_path = ".cache_".to_string() + tree_file_name;
    let thread_pool = AsyncComputeTaskPool::get();

    // Delete temporary file if present (most likely from an interrupted load operation)
    delete_by_path(&tmp_file_path);

    TreeLoadingTask {
        confirmed,
        tmp_file_path,
        target_cache_file_path,
        model_name: tree_file_name.to_string(),
        payload: TreePayload::Loading(thread_pool.spawn(async move {
            if Path::new(&tmp_file_path_).exists() {
                match BoxTree::load(&tmp_file_path_) {
                    Ok(tree) => Ok(tree),
                    Err(err) => Err(err.to_string()),
                }
            } else {
                let tree = BoxTree::<u32>::load_vox_file(model_path.as_str(), BRICK_DIMENSION);
                if let Ok(mut tree) = tree {
                    tree.albedo_mip_map_resampling_strategy()
                        .switch_albedo_mip_maps(true);
                    tree.save(&tmp_file_path_).ok().unwrap();
                }
                match BoxTree::load(&tmp_file_path_) {
                    Ok(tree) => Ok(tree),
                    Err(err) => Err(err.to_string()),
                }
            }
        })),
    }
}

pub(crate) fn observe_file_drop(
    mut commands: Commands,
    mut evr_dnd: EventReader<FileDragAndDrop>,
    mut tree_factory: Option<ResMut<TreeLoadingTask>>,
    mut status_text: Query<(&mut Text2d, &mut UiColor, &Model, &Loading)>,
) {
    let (mut message_text, mut message_color, _, _) = status_text
        .single_mut()
        .expect("Expected Status message to be available in UI");
    for ev in evr_dnd.read() {
        match ev {
            FileDragAndDrop::HoveredFile {
                window: _,
                path_buf,
            } => {
                *message_color = UiColor::from(Color::srgb(0.88, 0.62, 0.49));
                message_text.0 = "Initiated model load".to_string();
                commands.insert_resource(load_task_from_path(path_buf, false));
            }
            FileDragAndDrop::DroppedFile {
                window: _,
                path_buf,
            } => {
                let tree_facory = tree_factory
                    .as_mut()
                    .expect("Expected available tree loading task upon model load cancellation");
                #[cfg(debug_assertions)]
                {
                    let tree_file_name = path_buf
                        .file_stem()
                        .unwrap_or_else(|| OsStr::new("unknwon"))
                        .to_str()
                        .unwrap_or_else(|| "name_conversion_failed");
                    let tmp_file_path_ = ".tmp_cache_".to_string() + tree_file_name;
                    debug_assert!(tmp_file_path_ == tree_facory.tmp_file_path);
                }
                tree_facory.confirmed = true;
                message_text.0 = "Initiated model load".to_string();
            }
            FileDragAndDrop::HoveredFileCanceled { window: _ } => {
                debug_assert!(tree_factory.is_some() && !tree_factory.as_ref().unwrap().confirmed);
                delete_by_path(&tree_factory.as_ref().unwrap().tmp_file_path);
                commands.remove_resource::<TreeLoadingTask>();
                *message_color = UiColor::from(Color::srgb(0.2, 0.1, 0.25));
                message_text.0 = "Cancelled model load".to_string();
            }
        }
    }
}

pub(crate) fn handle_model_load_finished(
    mut commands: Commands,
    mut pkv: ResMut<PkvStore>,
    images: ResMut<Assets<Image>>,
    mut viewset: ResMut<VhxViewSet>,
    mut ui_state: ResMut<UiState>,
    tree_factory: Option<ResMut<TreeLoadingTask>>,
    mut view_output: Query<(&mut Sprite, &Model, &Output, &Container)>,
    mut status_text: Query<(&mut Text2d, &mut UiColor, &Model, &Loading)>,
    mut model_name: Query<
        (&mut Text2d, &Model, &Info),
        (Without<Status>, Without<Version>, Without<Loading>),
    >,
) {
    if let Some(mut tree_factory) = tree_factory {
        let (mut message_text, mut message_color, _, _) = status_text
            .single_mut()
            .expect("Expected Status message to be available in UI");
        if tree_factory.confirmed {
            if let TreePayload::Loading(ref mut task) = tree_factory.payload {
                if let Some(tree) = block_on(future::poll_once(task)) {
                    if let Err(e) = tree {
                        message_text.0 = format!("Error during model load: {e}");
                        delete_by_path(&tree_factory.tmp_file_path);
                        delete_by_path(&tree_factory.target_cache_file_path);
                        return;
                    }
                    tree_factory.payload = TreePayload::Loaded(tree.ok().unwrap())
                }
                // Process the tree in next iteration
                message_text.0 = format!("Initiating GPU View..");
                return;
            }

            debug_assert!(matches!(tree_factory.payload, TreePayload::Loaded(_)));
            let tree = if let TreePayload::Loaded(tree) = std::mem::take(&mut tree_factory.payload)
            {
                tree
            } else {
                panic!("Expected tree payload!");
            };

            debug_assert!(
                Path::new(&tree_factory.tmp_file_path).exists(),
                "Expected {:?} to exist after tree load is completed!",
                tree_factory.tmp_file_path
            );

            let model_name_text =
                format!("{}({}^3)", tree_factory.model_name.clone(), tree.get_size());

            if tree_factory.tmp_file_path != tree_factory.target_cache_file_path {
                std::fs::rename(
                    &tree_factory.tmp_file_path,
                    &tree_factory.target_cache_file_path,
                )
                .expect("Expected to be able to remove temporary file at {tmp_file_path}");
            }

            let mut host = BoxTreeGPUHost { tree };
            viewset.clear();
            let view_index = host.create_new_view(
                &mut viewset,
                50,
                voxelhex::raytracing::Viewport::new(
                    V3c::new(0., 10., 0.),
                    V3c::new(0., 0., 1.),
                    V3c::new(
                        ui_state.viewport_resolution[0] as f32,
                        ui_state.viewport_resolution[1] as f32,
                        ui_state.view_distance as f32,
                    ),
                    ui_state.fov_value as f32,
                ),
                ui_state.output_resolution,
                images,
            );

            // Set output render as tree view output
            let (mut output_sprite, _, _, _) = view_output
                .single_mut()
                .expect("Expected to have model output image available in UI!");
            *output_sprite = Sprite::from_image(viewset.view(view_index).output_texture().clone());

            // Insert the tree resource
            ui_state.model_loaded = true;
            commands.insert_resource(host);
            pkv.set("last_loaded_model", &tree_factory.target_cache_file_path)
                .expect("Expected to be able to store last_loaded_model setting");

            // Set models name
            let (mut ui_model_name_text, _, _) = model_name
                .single_mut()
                .expect("Expected Model Name to be available in UI");

            // Extend name with spaces until 40 characters
            let spaces_needed = 40 - model_name_text.len().min(40);
            let spacer = (0..(spaces_needed / 2)).map(|_| " ").collect::<String>();
            ui_model_name_text.0 = format!("{}{}{}", spacer, model_name_text, spacer);

            *message_color = UiColor::from(Color::srgb(0.2, 0.1, 0.25));

            if tree_factory.tmp_file_path != tree_factory.target_cache_file_path {
                message_text.0 = "Finished loading model!".to_string();
            } else {
                message_text.0 = "Opened last loaded model!".to_string();
            }
            commands.remove_resource::<TreeLoadingTask>();
        }
    }
}

pub(crate) fn load_last_loaded_model(
    pkv: Res<PkvStore>,
    mut commands: Commands,
    ui_state: ResMut<UiState>,
    tree_factory: Option<Res<TreeLoadingTask>>,
    mut status_text: Query<(&mut Text2d, &mut UiColor, &Model, &Loading)>,
) {
    if !ui_state.model_loaded && tree_factory.is_none() {
        let (mut message_text, mut message_color, _, _) = status_text
            .single_mut()
            .expect("Expected Status message to be available in UI");

        *message_color = UiColor::from(Color::srgb(0.88, 0.62, 0.49));

        if let Ok(file_path) = pkv.get::<String>("last_loaded_model") {
            if Path::new(&file_path).exists() {
                // Last successful model can be parsed directly
                message_text.0 = "Trying to parse last loaded model..".to_string();

                let thread_pool = AsyncComputeTaskPool::get();
                let file_path_ = file_path.to_string();

                let task = thread_pool.spawn(async move {
                    match BoxTree::load(&file_path) {
                        Ok(tree) => Ok(tree),
                        Err(err) => Err(err.to_string()),
                    }
                });

                commands.insert_resource(TreeLoadingTask {
                    confirmed: true,
                    model_name: file_path_
                        .split(".cache_")
                        .last()
                        .map(str::trim)
                        .unwrap_or(&file_path_)
                        .to_string(),
                    payload: TreePayload::Loading(task),
                    tmp_file_path: file_path_.to_string(),
                    target_cache_file_path: file_path_,
                });
                return;
            }
        }

        // Couldn't load previously loaded model, load default model
        message_text.0 = "Loading default model..".to_string();
        let default_model_path =
            PathBuf::from("whisp/assets/models/gingerbread_house_by_kirra_luan.vox");

        commands.insert_resource(load_task_from_path(&default_model_path, true));
    }
}
