use crate::{
    boxtree::{
        iterate::execute_for_relevant_sectants,
        types::{BrickData, NodeContent},
        BoxTree, V3c, V3cf32, VoxelData, BOX_NODE_CHILDREN_COUNT, BOX_NODE_DIMENSION,
    },
    object_pool::empty_marker,
    raytracing::bevy::{
        create_depth_texture, create_output_texture,
        types::{
            BoxTreeGPUDataHandler, BoxTreeGPUHost, BoxTreeGPUView, BoxTreeMetaData,
            BoxTreeRenderData, BoxTreeSpyGlass, BrickOwnedBy, BrickUploadRequest,
            NodeUploadRequest, UploadQueueStatus, UploadQueueTargets, VhxRenderPipeline,
            VhxViewSet, VictimPointer, Viewport,
        },
    },
    spatial::Cube,
};
use bendy::{decoding::FromBencode, encoding::ToBencode};
use bevy::{
    ecs::system::{Res, ResMut},
    math::Vec4,
    prelude::{Assets, Image},
    render::{
        render_resource::{
            encase::{internal::WriteInto, UniformBuffer},
            Buffer, ShaderSize,
        },
        renderer::{RenderDevice, RenderQueue},
    },
};
use bimap::BiHashMap;
use std::{
    collections::HashSet,
    hash::Hash,
    ops::Range,
    sync::{Arc, RwLock},
};

fn boxtree_properties<
    #[cfg(all(feature = "bytecode", feature = "serialization"))] T: FromBencode
        + ToBencode
        + Serialize
        + DeserializeOwned
        + Default
        + Eq
        + Clone
        + Hash
        + VoxelData,
    #[cfg(all(feature = "bytecode", not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData,
    #[cfg(all(not(feature = "bytecode"), feature = "serialization"))] T: Serialize + DeserializeOwned + Default + Eq + Clone + Hash + VoxelData,
    #[cfg(all(not(feature = "bytecode"), not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData,
>(
    tree: &BoxTree<T>,
) -> u32 {
    (tree.brick_dim & 0x0000FFFF) | ((tree.mip_map_strategy.is_enabled() as u32) << 16)
}

impl<
        #[cfg(all(feature = "bytecode", feature = "serialization"))] T: FromBencode
            + ToBencode
            + Serialize
            + DeserializeOwned
            + Default
            + Eq
            + Clone
            + Hash
            + VoxelData
            + Send
            + Sync
            + 'static,
        #[cfg(all(feature = "bytecode", not(feature = "serialization")))] T: FromBencode + ToBencode + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
        #[cfg(all(not(feature = "bytecode"), feature = "serialization"))] T: Serialize
            + DeserializeOwned
            + Default
            + Eq
            + Clone
            + Hash
            + VoxelData
            + Send
            + Sync
            + 'static,
        #[cfg(all(not(feature = "bytecode"), not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    > BoxTreeGPUHost<T>
{
    //##############################################################################
    //     ███████      █████████  ███████████ ███████████   ██████████ ██████████
    //   ███░░░░░███   ███░░░░░███░█░░░███░░░█░░███░░░░░███ ░░███░░░░░█░░███░░░░░█
    //  ███     ░░███ ███     ░░░ ░   ░███  ░  ░███    ░███  ░███  █ ░  ░███  █ ░
    // ░███      ░███░███             ░███     ░██████████   ░██████    ░██████
    // ░███      ░███░███             ░███     ░███░░░░░███  ░███░░█    ░███░░█
    // ░░███     ███ ░░███     ███    ░███     ░███    ░███  ░███ ░   █ ░███ ░   █
    //  ░░░███████░   ░░█████████     █████    █████   █████ ██████████ ██████████
    //    ░░░░░░░      ░░░░░░░░░     ░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░ ░░░░░░░░░░
    //    █████████  ███████████  █████  █████
    //   ███░░░░░███░░███░░░░░███░░███  ░░███
    //  ███     ░░░  ░███    ░███ ░███   ░███
    // ░███          ░██████████  ░███   ░███
    // ░███    █████ ░███░░░░░░   ░███   ░███
    // ░░███  ░░███  ░███         ░███   ░███
    //  ░░█████████  █████        ░░████████
    //   ░░░░░░░░░  ░░░░░          ░░░░░░░░
    //  █████   █████ █████ ██████████ █████   ███   █████
    // ░░███   ░░███ ░░███ ░░███░░░░░█░░███   ░███  ░░███
    //  ░███    ░███  ░███  ░███  █ ░  ░███   ░███   ░███
    //  ░███    ░███  ░███  ░██████    ░███   ░███   ░███
    //  ░░███   ███   ░███  ░███░░█    ░░███  █████  ███
    //   ░░░█████░    ░███  ░███ ░   █  ░░░█████░█████░
    //     ░░███      █████ ██████████    ░░███ ░░███
    //##############################################################################

    /// Creates GPU compatible data renderable on the GPU from an BoxTree
    pub fn create_new_view(
        &mut self,
        viewset: &mut VhxViewSet,
        viewport: Viewport,
        resolution: [u32; 2],
        mut images: ResMut<Assets<Image>>,
    ) -> usize {
        let tree = &self.tree;

        // This is an estimation for the required number of nodes in the given view
        // which sums the number of nodes within the viewport on every level
        let nodes_in_view: usize = {
            let tree_max_levels = (tree.boxtree_size as f32 / tree.brick_dim as f32)
                .log(4.)
                .ceil() as u32;
            (1..=tree_max_levels)
                .map(|level| {
                    let view_distance = viewport.frustum.z;
                    let cube_size_at_level = tree.brick_dim * 4_u32.pow(level);
                    (view_distance / cube_size_at_level as f32).ceil().powf(3.) as usize
                })
                .sum()
        };

        let bricks_in_view = (
            // Bricks in view
            ((viewport.frustum.z / tree.brick_dim as f32).ceil() as usize).pow(3)
                // MIPs in view
                + nodes_in_view
        ) / 4;

        let gpu_data_handler = BoxTreeGPUDataHandler {
            upload_range: Cube {
                min_position: viewport.origin - V3c::unit(viewport.frustum.z / 2.),
                size: viewport.frustum.z,
            },
            render_data: BoxTreeRenderData {
                mips_enabled: self.tree.mip_map_strategy.is_enabled(),
                boxtree_meta: BoxTreeMetaData {
                    boxtree_size: self.tree.boxtree_size,
                    tree_properties: boxtree_properties(&self.tree),
                    ambient_light_color: V3c::new(1., 1., 1.),
                    ambient_light_position: V3c::new(
                        self.tree.boxtree_size as f32,
                        self.tree.boxtree_size as f32,
                        self.tree.boxtree_size as f32,
                    ),
                },
                node_metadata: vec![0; (nodes_in_view as f32 / 8.).ceil() as usize],
                node_ocbits: vec![0; nodes_in_view * 2],
                node_children: vec![empty_marker(); nodes_in_view * BOX_NODE_CHILDREN_COUNT],
                node_mips: vec![empty_marker(); nodes_in_view],
                color_palette: vec![Vec4::ZERO; u16::MAX as usize],
            },
            upload_targets: UploadQueueTargets {
                node_upload_queue: vec![],
                brick_upload_queue: vec![],
                brick_ownership: BiHashMap::new(),
                brick_positions: vec![V3c::unit(0.); bricks_in_view],
                node_key_vs_meta_index: BiHashMap::new(),
                nodes_to_see: HashSet::new(),
            },
            upload_state: UploadQueueStatus {
                victim_node: VictimPointer::new(nodes_in_view),
                victim_brick: 0,
                node_upload_progress: 0,
                brick_upload_progress: 0,
                uploaded_color_palette_size: 0,
            },
            nodes_in_view,
            bricks_in_view,
            node_uploads_per_frame: 4,
            brick_uploads_per_frame: 4,
            brick_unload_search_perimeter: 8,
        };
        let output_texture = create_output_texture(resolution, &mut images);

        viewset.views.push(Arc::new(RwLock::new(BoxTreeGPUView {
            resolution,
            reload: true,
            rebuild: false,
            resize: false,
            new_images_ready: true,
            new_resolution: None,
            new_output_texture: None,
            new_depth_texture: None,
            data_handler: gpu_data_handler,
            resources: None,
            brick_slot: Cube::brick_slot_for(&viewport.origin, tree.brick_dim),
            spyglass: BoxTreeSpyGlass {
                depth_texture: create_depth_texture(resolution, &mut images),
                output_texture,
                viewport_changed: true,
                viewport,
            },
        })));
        viewset.changed = true;
        viewset.views.len() - 1
    }
}

//##############################################################################
//  █████  █████ ███████████  █████          ███████      █████████   ██████████
// ░░███  ░░███ ░░███░░░░░███░░███         ███░░░░░███   ███░░░░░███ ░░███░░░░███
//  ░███   ░███  ░███    ░███ ░███        ███     ░░███ ░███    ░███  ░███   ░░███
//  ░███   ░███  ░██████████  ░███       ░███      ░███ ░███████████  ░███    ░███
//  ░███   ░███  ░███░░░░░░   ░███       ░███      ░███ ░███░░░░░███  ░███    ░███
//  ░███   ░███  ░███         ░███      █░░███     ███  ░███    ░███  ░███    ███
//  ░░████████   █████        ███████████ ░░░███████░   █████   █████ ██████████
//   ░░░░░░░░   ░░░░░        ░░░░░░░░░░░    ░░░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░
//   █████████    █████████  █████   █████ ██████████ ██████████   █████  █████ █████       ██████████
//  ███░░░░░███  ███░░░░░███░░███   ░░███ ░░███░░░░░█░░███░░░░███ ░░███  ░░███ ░░███       ░░███░░░░░█
// ░███    ░░░  ███     ░░░  ░███    ░███  ░███  █ ░  ░███   ░░███ ░███   ░███  ░███        ░███  █ ░
// ░░█████████ ░███          ░███████████  ░██████    ░███    ░███ ░███   ░███  ░███        ░██████
//  ░░░░░░░░███░███          ░███░░░░░███  ░███░░█    ░███    ░███ ░███   ░███  ░███        ░███░░█
//  ███    ░███░░███     ███ ░███    ░███  ░███ ░   █ ░███    ███  ░███   ░███  ░███      █ ░███ ░   █
// ░░█████████  ░░█████████  █████   █████ ██████████ ██████████   ░░████████   ███████████ ██████████
//  ░░░░░░░░░    ░░░░░░░░░  ░░░░░   ░░░░░ ░░░░░░░░░░ ░░░░░░░░░░     ░░░░░░░░   ░░░░░░░░░░░ ░░░░░░░░░░
//##############################################################################
impl UploadQueueTargets {
    pub(crate) fn reset(&mut self) {
        self.node_upload_queue.clear();
        self.brick_upload_queue.clear();
        self.brick_ownership.clear();
        self.node_key_vs_meta_index.clear();
        self.nodes_to_see.clear();
    }
}

pub(crate) fn handle_upload_queue_changes<
    #[cfg(all(feature = "bytecode", feature = "serialization"))] T: FromBencode
        + ToBencode
        + Serialize
        + DeserializeOwned
        + Default
        + Eq
        + Clone
        + Hash
        + VoxelData
        + Send
        + Sync
        + 'static,
    #[cfg(all(feature = "bytecode", not(feature = "serialization")))] T: FromBencode + ToBencode + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), feature = "serialization"))] T: Serialize + DeserializeOwned + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
>(
    tree_gpu_host: Option<Res<BoxTreeGPUHost<T>>>,
    mut vhx_pipeline: Option<ResMut<VhxRenderPipeline>>,
    mut viewset: Option<ResMut<VhxViewSet>>,
) {
    let (Some(pipeline), Some(tree_host), Some(viewset)) = (
        vhx_pipeline.as_mut(),
        tree_gpu_host.as_ref(),
        viewset.as_mut(),
    ) else {
        return; // Nothing to do without the required resources
    };

    if viewset.is_empty() {
        return; // Nothing to do without views..
    }

    let mut view = viewset.view_mut(0).unwrap();
    let mut updates = vec![];
    let mut ocbits_updated = usize::MAX..0;

    'uploading_buffers: {
        // Decide upload targets
        if view.reload {
            rebuild_upload_queues::<T>(
                &tree_host.tree,
                &view.spyglass.viewport.origin.clone(),
                view.spyglass.viewport.frustum.z,
                &mut view.data_handler.upload_targets,
            );

            view.data_handler.upload_state.node_upload_progress = 0;
            view.data_handler.upload_state.brick_upload_progress = 0;

            view.reload = false;
        }

        // Upload targets if view is ready
        if !view.reload {
            let data_handler = &mut view.data_handler;

            debug_assert!(
                data_handler.upload_state.node_upload_progress
                    <= data_handler.upload_targets.node_upload_queue.len()
            );

            // Handle node uploads
            for _ in 0..data_handler.node_uploads_per_frame.min(
                data_handler.upload_targets.node_upload_queue.len()
                    - data_handler.upload_state.node_upload_progress,
            ) {
                // find first node to upload
                while data_handler.upload_state.node_upload_progress
                    < data_handler.upload_targets.node_upload_queue.len()
                {

                    if let Some(node_meta_index) = data_handler
                        .upload_targets
                        .node_key_vs_meta_index
                        .get_by_left(&node_upload_request.node_key)
                        .cloned()
                    {
                        // Skip to next node if the current node is already uploaded
                        if matches!(
                            tree_host.tree.node_mips[node_upload_request.node_key],
                            BrickData::Parted(_)
                        ) && data_handler.render_data.node_mips[node_meta_index]
                            == empty_marker::<u32>()
                        {
                            // Upload MIP again, if not present already
                            let mip_update = data_handler.add_brick(
                                &tree_host.tree,
                                BrickUploadRequest {
                                    ownership: BrickOwnedBy::NodeAsMIP(
                                        node_upload_request.node_key as u32,
                                    ),
                                    min_position: V3c::default(),
                                },
                            );
                            if mip_update.allocation_failed {
                                // Can't fit new mip brick into buffers, need to rebuild the pipeline
                                re_evaluate_view_size(&mut view);
                                break 'uploading_buffers; // voxel data still needs to be written out
                            }
                            updates.push(mip_update);
                        }

                        data_handler.upload_state.node_upload_progress += 1;
                        continue;
                    }

                    // Upload Node to GPU
                    let (new_node_index, new_node_update) =
                        data_handler.add_node(&tree_host.tree, &node_upload_request);

                    if new_node_update.allocation_failed {
                        // Can't fit new brick into buffers, need to rebuild the pipeline
                        re_evaluate_view_size(&mut view);
                        break 'uploading_buffers; // voxel data still needs to be written out
                    }
                    updates.push(new_node_update);

                    // Upload MIP to GPU
                    let mip_update = data_handler.add_brick(
                        &tree_host.tree,
                        BrickUploadRequest {
                            ownership: BrickOwnedBy::NodeAsMIP(node_upload_request.node_key as u32),
                            min_position: V3c::unit(0.), // min_position not used for MIPs
                        },
                    );

                    if mip_update.allocation_failed {
                        // Can't fit new MIP brick into buffers, need to rebuild the pipeline
                        re_evaluate_view_size(&mut view);
                        break 'uploading_buffers; // voxel data still needs to be written out
                    }
                    updates.push(mip_update);

                    // Also set the ocbits updated range
                    ocbits_updated.start = ocbits_updated.start.min(new_node_index * 2);
                    ocbits_updated.end = ocbits_updated.end.max(new_node_index * 2 + 2);

                    data_handler.upload_state.node_upload_progress += 1;
                    break;
                }
                if data_handler.upload_state.node_upload_progress
                    == data_handler.upload_targets.node_upload_queue.len()
                {
                    // No more nodes to upload!
                    break;
                }
            }


            // Handle brick uploads
            if data_handler.upload_state.brick_upload_progress
                == data_handler.upload_targets.brick_upload_queue.len()
            {
                break 'uploading_buffers; // All bricks are already uploaded

            }
            for _ in 0..data_handler.brick_uploads_per_frame {
                debug_assert!(
                    data_handler.upload_state.brick_upload_progress
                        < data_handler.upload_targets.brick_upload_queue.len()
                );

                // find a brick to upload
                for brick_request_index in data_handler.upload_state.brick_upload_progress
                    ..data_handler.upload_targets.brick_upload_queue.len()
                {
                    let brick_request =
                        data_handler.upload_targets.brick_upload_queue[brick_request_index].clone();
                    let brick_ownership = brick_request.ownership.clone();

                    if
                    // current brick is not uploaded
                    data_handler
                        .upload_targets
                        .brick_ownership
                        .get_by_right(&brick_request.ownership)
                        .is_none()
                        // current brick can be uploaded
                        && match brick_request.ownership {
                            BrickOwnedBy::None => panic!("Request to upload unowned brick?!"),
                            BrickOwnedBy::NodeAsMIP(node_key) |BrickOwnedBy::NodeAsChild(node_key, _) => {
                                // Brick can be uploaded if its parent is already uploaded to the GPU
                                 data_handler.upload_targets.node_key_vs_meta_index.contains_left(&(node_key as usize))
                                 // Brick should be uploaded if its parent also needs to be uploaded to GPU
                                 && data_handler.upload_targets.nodes_to_see.contains(&(node_key as usize))
                            }
                        }
                    {
                        let brick_update = data_handler.add_brick(&tree_host.tree, brick_request);
                        if brick_update.allocation_failed {
                            // Can't fit new brick into buffers, need to rebuild the pipeline
                            re_evaluate_view_size(&mut view);
                            break 'uploading_buffers; // voxel data still needs to be written out
                        }
                        updates.push(brick_update);
                    } else {
                        if brick_request_index
                            == data_handler.upload_targets.brick_upload_queue.len()
                        {
                            break 'uploading_buffers; // Can't upload more bricks this loop
                        }
                        continue;
                    }

                    // In case current brick request is uploaded already, just increase progress
                    debug_assert!(data_handler
                        .upload_targets
                        .brick_ownership
                        .get_by_right(&brick_ownership)
                        .is_some());
                    if brick_request_index == data_handler.upload_state.brick_upload_progress {
                        data_handler.upload_state.brick_upload_progress += 1;
                        if data_handler.upload_state.brick_upload_progress
                            == data_handler.upload_targets.brick_upload_queue.len()
                        {
                            break 'uploading_buffers; // No more bricks to upload
                        }
                    }
                    break;
                }
            }
        }
    }


    if view.resources.is_none() {
        return; // Can't write to buffers as there are not created
    }

    // Apply writes to GPU
    let render_queue = &pipeline.render_queue;

    // Data updates for spyglass viewport
    if view.spyglass.viewport_changed {
        view.spyglass.viewport_changed = false;

        let mut buffer = UniformBuffer::new(Vec::<u8>::new());
        buffer.write(&view.spyglass.viewport).unwrap();
        render_queue.write_buffer(
            &view.resources.as_ref().unwrap().viewport_buffer,
            0,
            &buffer.into_inner(),
        );
    }

    // Data updates for BoxTree MIP map feature
    let tree = &tree_host.tree;
    if view.data_handler.render_data.mips_enabled != tree.mip_map_strategy.is_enabled() {
        // Regenerate feature bits
        view.data_handler.render_data.boxtree_meta.tree_properties = boxtree_properties(tree);

        // Write to GPU
        let mut buffer = UniformBuffer::new(Vec::<u8>::new());
        buffer
            .write(&view.data_handler.render_data.boxtree_meta)
            .unwrap();
        pipeline.render_queue.write_buffer(
            &view.resources.as_ref().unwrap().node_metadata_buffer,
            0,
            &buffer.into_inner(),
        );
        view.data_handler.render_data.mips_enabled = tree.mip_map_strategy.is_enabled()
    }

    // Data updates for color palette
    let host_color_count = tree.map_to_color_index_in_palette.keys().len();
    let color_palette_size_diff =
        host_color_count - view.data_handler.upload_state.uploaded_color_palette_size;

    debug_assert!(
        host_color_count >= view.data_handler.upload_state.uploaded_color_palette_size,
        "Expected host color palette({:?}), to be larger, than colors stored on the GPU({:?})",
        host_color_count,
        view.data_handler.upload_state.uploaded_color_palette_size
    );

    if 0 < color_palette_size_diff {
        for i in view.data_handler.upload_state.uploaded_color_palette_size..host_color_count {
            view.data_handler.render_data.color_palette[i] = tree.voxel_color_palette[i].into();
        }

        // Upload color palette delta to GPU
        write_range_to_buffer(
            &view.data_handler.render_data.color_palette,
            (host_color_count - color_palette_size_diff)..(host_color_count),
            &view.resources.as_ref().unwrap().color_palette_buffer,
            render_queue,
        );
    }
    view.data_handler.upload_state.uploaded_color_palette_size =
        tree.map_to_color_index_in_palette.keys().len();

    // compile cache updates into write batches
    let mut node_meta_updated = usize::MAX..0;
    let mut node_children_updated = usize::MAX..0;
    let mut node_mips_updated = usize::MAX..0; // Any brick upload could invalidate node_mips values
    for cache_update in updates.into_iter() {
        for meta_index in cache_update.modified_nodes {
            node_meta_updated.start = node_meta_updated.start.min(meta_index / 8);
            node_meta_updated.end = node_meta_updated.end.max(meta_index / 8 + 1);
            node_mips_updated.start = node_mips_updated.start.min(meta_index);
            node_mips_updated.end = node_mips_updated.end.max(meta_index + 1);
            node_children_updated.start = node_children_updated
                .start
                .min(meta_index * BOX_NODE_CHILDREN_COUNT);
            node_children_updated.end = node_children_updated
                .end
                .max(meta_index * BOX_NODE_CHILDREN_COUNT + BOX_NODE_CHILDREN_COUNT);
        }

        // Upload Voxel data
        if let Some(modified_brick_data) = cache_update.brick_update {
            let voxel_start_index =
                modified_brick_data.brick_index * modified_brick_data.data.len();
            debug_assert_eq!(
                modified_brick_data.data.len(),
                tree.brick_dim.pow(3) as usize,
                "Expected Brick slice to align to tree brick dimension"
            );
            unsafe {
                render_queue.write_buffer(
                    &view.resources.as_ref().unwrap().voxels_buffer,
                    (voxel_start_index * std::mem::size_of_val(&modified_brick_data.data[0]))
                        as u64,
                    modified_brick_data.data.align_to::<u8>().1,
                );
            }
        }
    }


    write_range_to_buffer(
        &view.data_handler.render_data.node_metadata,
        node_meta_updated,
        &view.resources.as_ref().unwrap().node_metadata_buffer,
        render_queue,
    );
    write_range_to_buffer(
        &view.data_handler.render_data.node_children,
        node_children_updated,
        &view.resources.as_ref().unwrap().node_children_buffer,
        render_queue,
    );
    write_range_to_buffer(
        &view.data_handler.render_data.node_ocbits,
        ocbits_updated,
        &view.resources.as_ref().unwrap().node_ocbits_buffer,
        render_queue,
    );
    write_range_to_buffer(
        &view.data_handler.render_data.node_mips,
        node_mips_updated,
        &view.resources.as_ref().unwrap().node_mips_buffer,
        render_queue,
    );
}

//##############################################################################
//  █████  █████ ███████████  █████          ███████      █████████   ██████████
// ░░███  ░░███ ░░███░░░░░███░░███         ███░░░░░███   ███░░░░░███ ░░███░░░░███
//  ░███   ░███  ░███    ░███ ░███        ███     ░░███ ░███    ░███  ░███   ░░███
//  ░███   ░███  ░██████████  ░███       ░███      ░███ ░███████████  ░███    ░███
//  ░███   ░███  ░███░░░░░░   ░███       ░███      ░███ ░███░░░░░███  ░███    ░███
//  ░███   ░███  ░███         ░███      █░░███     ███  ░███    ░███  ░███    ███
//  ░░████████   █████        ███████████ ░░░███████░   █████   █████ ██████████
//   ░░░░░░░░   ░░░░░        ░░░░░░░░░░░    ░░░░░░░    ░░░░░   ░░░░░ ░░░░░░░░░░
//     ██████    █████  █████ ██████████ █████  █████ ██████████  █████████
//   ███░░░░███ ░░███  ░░███ ░░███░░░░░█░░███  ░░███ ░░███░░░░░█ ███░░░░░███
//  ███    ░░███ ░███   ░███  ░███  █ ░  ░███   ░███  ░███  █ ░ ░███    ░░░
// ░███     ░███ ░███   ░███  ░██████    ░███   ░███  ░██████   ░░█████████
// ░███   ██░███ ░███   ░███  ░███░░█    ░███   ░███  ░███░░█    ░░░░░░░░███
// ░░███ ░░████  ░███   ░███  ░███ ░   █ ░███   ░███  ░███ ░   █ ███    ░███
//  ░░░██████░██ ░░████████   ██████████ ░░████████   ██████████░░█████████
//##############################################################################
/// Invalidates view to be rebuilt on the size needed by bricks and nodes
pub(crate) fn re_evaluate_view_size(view: &mut BoxTreeGPUView) {
    // Decide if there's enough space to host the required number of nodes
    let nodes_needed_overall = view
        .data_handler
        .upload_targets
        .node_upload_queue
        .iter()
        .skip(view.data_handler.upload_state.node_upload_progress)
        .filter(|item| {
            !view
                .data_handler
                .upload_targets
                .node_key_vs_meta_index
                .contains_right(&item.node_key)
        })
        .count()
        + view
            .data_handler
            .upload_targets
            .node_key_vs_meta_index
            .len();
    let rebuild_nodes = nodes_needed_overall > view.data_handler.nodes_in_view;

    if rebuild_nodes {
        let new_node_count = (nodes_needed_overall as f32 * 1.1) as usize;
        let render_data = &mut view.data_handler.render_data;

        // Extend render data
        render_data
            .node_metadata
            .resize((new_node_count as f32 / 8.).ceil() as usize, 0);
        render_data
            .node_children
            .resize(new_node_count * BOX_NODE_CHILDREN_COUNT, empty_marker());
        render_data.node_mips.resize(new_node_count, empty_marker());
        render_data.node_ocbits.resize(new_node_count * 2, 0);
        view.data_handler.nodes_in_view = new_node_count;
        view.data_handler.upload_state.victim_node.max_meta_len = new_node_count;
    }

    // Decide if there's enough space to host the required number of bricks
    let bricks_needed_overall = view
        .data_handler
        .upload_targets
        .brick_upload_queue
        .iter()
        .skip(view.data_handler.upload_state.brick_upload_progress)
        .filter(|item| {
            !view
                .data_handler
                .upload_targets
                .brick_ownership
                .contains_right(&item.ownership)
        })
        .count()
        + view.data_handler.upload_targets.brick_ownership.len()
        + nodes_needed_overall;
    let rebuild_bricks = bricks_needed_overall > view.data_handler.bricks_in_view;
    if rebuild_bricks {
        let new_brick_count = (bricks_needed_overall as f32 * 1.1) as usize;
        view.data_handler.bricks_in_view = new_brick_count;
        view.data_handler
            .upload_targets
            .brick_positions
            .resize(new_brick_count, V3c::default());
    }


    debug_assert!(
        rebuild_nodes || rebuild_bricks,
        "Expected view to be too small while calling size evaluation!",
    );
    view.resize = true;
}

/// Recreates the list of nodes and bricks to upload based on the current position and view distance
pub(crate) fn rebuild_upload_queues<
    #[cfg(all(feature = "bytecode", feature = "serialization"))] T: FromBencode
        + ToBencode
        + Serializ
        + DeserializeOwned
        + Default
        + Eq
        + Clone
        + Hash
        + VoxelData
        + Send
        + Sync
        + 'static,
    #[cfg(all(feature = "bytecode", not(feature = "serialization")))] T: FromBencode + ToBencode + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), feature = "serialization"))] T: Serialize + DeserializeOwned + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
>(
    tree: &BoxTree<T>,
    viewport_center_: &V3cf32,
    view_distance: f32,
    upload_targets: &mut UploadQueueTargets,
) {
    upload_targets.nodes_to_see.clear();
    upload_targets.brick_upload_queue.clear();
    upload_targets.node_upload_queue.clear();

    // Determine view center range
    let viewport_center = V3c::new(
        viewport_center_.x.clamp(0., tree.boxtree_size as f32),
        viewport_center_.y.clamp(0., tree.boxtree_size as f32),
        viewport_center_.z.clamp(0., tree.boxtree_size as f32),
    );
    let viewport_bl_ = *viewport_center_ - V3c::unit(view_distance / 2.);
    let viewport_bl = V3c::new(
        viewport_bl_.x.clamp(0., tree.boxtree_size as f32),
        viewport_bl_.y.clamp(0., tree.boxtree_size as f32),
        viewport_bl_.z.clamp(0., tree.boxtree_size as f32),
    );
    let viewport_tr = viewport_bl + V3c::unit(view_distance);
    let viewport_tr = V3c::new(
        viewport_tr.x.clamp(0., tree.boxtree_size as f32),
        viewport_tr.y.clamp(0., tree.boxtree_size as f32),
        viewport_tr.z.clamp(0., tree.boxtree_size as f32),
    );

    // Decide the level boundaries to work within
    let max_mip_level = (tree.boxtree_size as f32 / tree.brick_dim as f32)
        .log(4.)
        .ceil() as i32;
    let deepest_mip_level_to_upload = ((viewport_bl_ - viewport_bl).length() / view_distance)
        .ceil()
        .min(max_mip_level as f32) as i32;

    // Look for the smallest node covering the entirety of the viewing distance
    let mut center_node_parent_key = None;
    let mut node_bounds = Cube::root_bounds(tree.boxtree_size as f32);
    let mut node_stack = vec![NodeUploadRequest {
        node_key: BoxTree::<T>::ROOT_NODE_KEY as usize,
        parent_key: BoxTree::<T>::ROOT_NODE_KEY as usize,
        sectant: 0,
    }];
    loop {
        let node_key = node_stack.last().unwrap().node_key;
        if
        // current node is either leaf or empty
        matches!(tree.nodes.get(node_key), NodeContent::Nothing | NodeContent::Leaf(_) | NodeContent::UniformLeaf(_))
        // or target child boundaries don't cover view distance
        || (node_bounds.size / BOX_NODE_DIMENSION as f32) <= view_distance
        || !node_bounds.contains(&viewport_bl)
        || !node_bounds.contains(&viewport_tr)
        {
            break;
        }

        // Hash the position to the target child
        let child_sectant_at_position = node_bounds.sectant_for(&viewport_center);
        let child_key_at_position =
            tree.node_children[node_key].child(child_sectant_at_position) as usize;

        // There is a valid child at the given position inside the node, recurse into it
        if tree.nodes.key_is_valid(child_key_at_position as usize) {
            center_node_parent_key = Some((node_key, node_bounds));
            node_stack.push(NodeUploadRequest {
                node_key: child_key_at_position,
                parent_key: node_key,
                sectant: child_sectant_at_position,
            });
            node_bounds = Cube::child_bounds_for(&node_bounds, child_sectant_at_position);
        } else {
            break;
        }
    }

    // Add parent and children nodes into the upload queue and view set
    let center_node_key = node_stack.last().unwrap().node_key.clone();
    upload_targets.node_upload_queue.append(
        &mut node_stack
            .drain(..)
            .map(|v| {
                upload_targets.nodes_to_see.insert(v.node_key);
                v
            })
            .collect(),
    );

    // add center node together with children inside the viewport into the queue
    add_children_to_upload_queue(
        (center_node_key, node_bounds),
        tree,
        &V3c::from(viewport_bl),
        view_distance,
        upload_targets,
        max_mip_level - deepest_mip_level_to_upload + 1,
    );
    // add center node direct siblings into upload queue
    let extended_range_modifier = 0.5;
    add_children_to_upload_queue(
        center_node_parent_key.unwrap_or((center_node_key, node_bounds)),
        tree,
        &V3c::from(viewport_bl - V3c::unit(view_distance * -extended_range_modifier)),
        view_distance * extended_range_modifier,
        upload_targets,
        1,
    );
}

fn add_children_to_upload_queue<
    #[cfg(all(feature = "bytecode", feature = "serialization"))] T: FromBencode
        + ToBencode
        + Serializ
        + DeserializeOwned
        + Default
        + Eq
        + Clone
        + Hash
        + VoxelData
        + Send
        + Sync
        + 'static,
    #[cfg(all(feature = "bytecode", not(feature = "serialization")))] T: FromBencode + ToBencode + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), feature = "serialization"))] T: Serialize + DeserializeOwned + Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
    #[cfg(all(not(feature = "bytecode"), not(feature = "serialization")))] T: Default + Eq + Clone + Hash + VoxelData + Send + Sync + 'static,
>(
    (node_key, node_bounds): (usize, Cube),
    tree: &BoxTree<T>,
    viewport_bl: &V3c<u32>,
    view_distance: f32,
    upload_targets: &mut UploadQueueTargets,
    depth_left: i32,
) {
    if depth_left <= 0 {
        return;
    }


    let viewport_contains_target_fn = |viewport_bl: &V3c<u32>,
                                       view_distance: f32,
                                       start_position_in_target: &V3c<u32>,
                                       update_size_in_target: &V3c<u32>|
     -> bool {
        !((viewport_bl.x + view_distance as u32) <= start_position_in_target.x
            || (start_position_in_target.x + update_size_in_target.x) <= viewport_bl.x
            || (viewport_bl.y + view_distance as u32) <= start_position_in_target.y
            || (start_position_in_target.y + update_size_in_target.y) <= viewport_bl.y
            || (viewport_bl.z + view_distance as u32) <= start_position_in_target.z
            || (start_position_in_target.z + update_size_in_target.z) <= viewport_bl.z)
    };

    debug_assert!(
        upload_targets.nodes_to_see.contains(&node_key),
        "Expected node to be already included in the upload queue"
    );
    match tree.nodes.get(node_key) {
        NodeContent::Nothing => {}
        NodeContent::UniformLeaf(brick) => {
            match &brick {
                BrickData::Empty | BrickData::Solid(_) => {
                    // Empty brickdata is not uploaded,
                    // while solid brickdata should be present in the nodes data
                }
                BrickData::Parted(_brick) => {
                    let brick_ownership = BrickOwnedBy::NodeAsChild(node_key as u32, 0);
                    if viewport_contains_target_fn(
                        viewport_bl,
                        view_distance,
                        &V3c::from(node_bounds.min_position),
                        &V3c::unit(node_bounds.size as u32),
                    ) && upload_targets
                        .brick_ownership
                        .get_by_right(&brick_ownership)
                        .is_none()
                    {
                        upload_targets.brick_upload_queue.push(BrickUploadRequest {
                            ownership: brick_ownership,
                            min_position: node_bounds.min_position,
                        });
                    }
                }
            };
        }
        NodeContent::Leaf(bricks) => {
            execute_for_relevant_sectants(
                &node_bounds,
                viewport_bl,
                view_distance as u32,
                |position_in_target,
                 update_size_in_target,
                 target_child_sectant,
                 &target_bounds| {
                    match &bricks[target_child_sectant as usize] {
                        BrickData::Empty | BrickData::Solid(_) => {
                            // Empty brickdata is not uploaded,
                            // while solid brickdata should be present in the nodes data
                        }
                        BrickData::Parted(_brick) => {
                            let brick_ownership =
                                BrickOwnedBy::NodeAsChild(node_key as u32, target_child_sectant);
                            if viewport_contains_target_fn(
                                viewport_bl,
                                view_distance,
                                &position_in_target,
                                &update_size_in_target,
                            ) && upload_targets
                                .brick_ownership
                                .get_by_right(&brick_ownership)
                                .is_none()
                            {
                                upload_targets.brick_upload_queue.push(BrickUploadRequest {
                                    ownership: brick_ownership,
                                    min_position: target_bounds.min_position,
                                });
                            }
                        }
                    };
                },
            );
        }
        NodeContent::Internal(_ocbits) => {
            execute_for_relevant_sectants(
                &node_bounds,
                viewport_bl,
                view_distance as u32,
                |position_in_target,
                 update_size_in_target,
                 target_child_sectant,
                 &target_bounds| {
                    if let Some(child_key) = tree.valid_child_for(node_key, target_child_sectant) {
                        if viewport_contains_target_fn(
                            viewport_bl,
                            view_distance,
                            &position_in_target,
                            &update_size_in_target,
                        ) {
                            upload_targets.node_upload_queue.push(NodeUploadRequest {
                                node_key: child_key,
                                parent_key: node_key,
                                sectant: target_child_sectant,
                            });
                            upload_targets.nodes_to_see.insert(child_key);
                            add_children_to_upload_queue(
                                (child_key, target_bounds),
                                tree,
                                &viewport_bl,
                                view_distance,
                                upload_targets,
                                depth_left - 1,
                            );
                        }
                    }
                },
            );
        }
    }
}

//##############################################################################
//    █████████  ███████████  █████  █████
//   ███░░░░░███░░███░░░░░███░░███  ░░███
//  ███     ░░░  ░███    ░███ ░███   ░███
// ░███          ░██████████  ░███   ░███
// ░███    █████ ░███░░░░░░   ░███   ░███
// ░░███  ░░███  ░███         ░███   ░███
//  ░░█████████  █████        ░░████████
//   ░░░░░░░░░  ░░░░░          ░░░░░░░░
//  ███████████   ██████████   █████████   ██████████
// ░░███░░░░░███ ░░███░░░░░█  ███░░░░░███ ░░███░░░░███
//  ░███    ░███  ░███  █ ░  ░███    ░███  ░███   ░░███
//  ░██████████   ░██████    ░███████████  ░███    ░███
//  ░███░░░░░███  ░███░░█    ░███░░░░░███  ░███    ░███
//  ░███    ░███  ░███ ░   █ ░███    ░███  ░███    ███
//  █████   █████ ██████████ █████   █████ ██████████
// ░░░░░   ░░░░░ ░░░░░░░░░░ ░░░░░   ░░░░░ ░░░░░░░░░░
//##############################################################################
fn read_buffer(
    render_device: &RenderDevice,
    buffer: &Buffer,
    index_range: std::ops::Range<usize>,
    target: &mut Vec<u32>,
) {
    let byte_start = (index_range.start * std::mem::size_of::<u32>()) as u64;
    let byte_end = (index_range.end * std::mem::size_of::<u32>()) as u64;
    let metadata_buffer_slice = buffer.slice(byte_start..byte_end);
    let (s, metadata_recv) = crossbeam::channel::unbounded::<()>();
    metadata_buffer_slice.map_async(
        bevy::render::render_resource::MapMode::Read,
        move |d| match d {
            Ok(_) => s.send(()).expect("Failed to send map update"),
            Err(err) => panic!("Couldn't map buffer!: {err}"),
        },
    );

    render_device
        .poll(bevy::render::render_resource::Maintain::wait())
        .panic_on_timeout();
    metadata_recv
        .recv()
        .expect("Failed to receive the map_async message");
    {
        let buffer_view = metadata_buffer_slice.get_mapped_range();
        *target = buffer_view
            .chunks(std::mem::size_of::<u32>())
            .map(|chunk| u32::from_ne_bytes(chunk.try_into().expect("should be a u32")))
            .collect::<Vec<u32>>();
    }
    buffer.unmap();
}

//##############################################################################
//    █████████  ███████████  █████  █████
//   ███░░░░░███░░███░░░░░███░░███  ░░███
//  ███     ░░░  ░███    ░███ ░███   ░███
// ░███          ░██████████  ░███   ░███
// ░███    █████ ░███░░░░░░   ░███   ░███
// ░░███  ░░███  ░███         ░███   ░███
//  ░░█████████  █████        ░░████████
//   ░░░░░░░░░  ░░░░░          ░░░░░░░░

//  █████   ███   █████ ███████████   █████ ███████████ ██████████
// ░░███   ░███  ░░███ ░░███░░░░░███ ░░███ ░█░░░███░░░█░░███░░░░░█
//  ░███   ░███   ░███  ░███    ░███  ░███ ░   ░███  ░  ░███  █ ░
//  ░███   ░███   ░███  ░██████████   ░███     ░███     ░██████
//  ░░███  █████  ███   ░███░░░░░███  ░███     ░███     ░███░░█
//   ░░░█████░█████░    ░███    ░███  ░███     ░███     ░███ ░   █
//     ░░███ ░░███      █████   █████ █████    █████    ██████████
//      ░░░   ░░░      ░░░░░   ░░░░░ ░░░░░    ░░░░░    ░░░░░░░░░░
//##############################################################################

/// Converts the given array to `&[u8]` on the given range,
/// and schedules it to be written to the given buffer in the GPU
fn write_range_to_buffer<U>(
    array: &[U],
    index_range: Range<usize>,
    buffer: &Buffer,
    render_queue: &RenderQueue,
) where
    U: Send + Sync + 'static + ShaderSize + WriteInto,
{
    if !index_range.is_empty() {
        let element_size = std::mem::size_of_val(&array[0]);
        let byte_offset = (index_range.start * element_size) as u64;
        let slice = array.get(index_range.clone()).unwrap_or_else(|| {
            panic!(
                "{}",
                format!(
                    "Expected range {:?} to be in bounds of {:?}",
                    index_range,
                    array.len(),
                )
                .to_owned()
            )
        });
        unsafe {
            render_queue.write_buffer(buffer, byte_offset, slice.align_to::<u8>().1);
        }
    }
}
