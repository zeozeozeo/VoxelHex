mod iterate_tests {
    use crate::{
        boxtree::{iterate::execute_for_relevant_sectants, BOX_NODE_DIMENSION},
        spatial::{math::vector::V3c, Cube},
    };

    #[test]
    fn test_sectant_execution_aligned_single_within() {
        let confines = Cube::root_bounds(400.);
        let update_size = 20;
        execute_for_relevant_sectants(
            &confines,
            &V3c::unit(0),
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(target_child_sectant, 0);
                assert_eq!(target_bounds.min_position, V3c::unit(0.));
                assert_eq!(update_size_in_target.x, update_size);
                assert_eq!(update_size_in_target.y, update_size);
                assert_eq!(update_size_in_target.z, update_size);
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
            },
        );

        let execute_position = V3c::new(100, 0, 0);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(target_child_sectant, 1);
                assert_eq!(target_bounds.min_position, execute_position.into());
                assert_eq!(update_size_in_target.x, update_size);
                assert_eq!(update_size_in_target.y, update_size);
                assert_eq!(update_size_in_target.z, update_size);

                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
            },
        );
    }

    #[test]
    fn test_sectant_execution_aligned_single_bounds_smaller_position() {
        let confines = Cube {
            min_position: V3c::unit(400.),
            size: 400.,
        };
        let update_size = 20;
        execute_for_relevant_sectants(
            &confines,
            &V3c::unit(0),
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(target_child_sectant, 0);
                assert_eq!(target_bounds.min_position, V3c::unit(400.));
                assert_eq!(update_size_in_target.x, update_size);
                assert_eq!(update_size_in_target.y, update_size);
                assert_eq!(update_size_in_target.z, update_size);
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
            },
        );

        let execute_position = V3c::new(100, 500, 0);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, _target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(
                    target_bounds.min_position,
                    confines.min_position + V3c::new(0., 100., 0.)
                );
                assert_eq!(update_size_in_target.x, update_size);
                assert_eq!(update_size_in_target.y, update_size);
                assert_eq!(update_size_in_target.z, update_size);

                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
            },
        );
    }

    #[test]
    fn test_sectant_execution_single_target_with_smaller_position_aligned() {
        let confines = Cube {
            min_position: V3c::unit(400.),
            size: 400.,
        };
        let update_size = 450;
        let execute_position = V3c::unit(0);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(target_child_sectant, 0);
                assert_eq!(target_bounds.min_position, confines.min_position);
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(
                    update_size_in_target.x,
                    update_size - confines.min_position.x as u32
                );
                assert_eq!(
                    update_size_in_target.y,
                    update_size - confines.min_position.y as u32
                );
                assert_eq!(
                    update_size_in_target.z,
                    update_size - confines.min_position.z as u32
                );
            },
        );
    }

    #[test]
    fn test_sectant_execution_single_target_with_smaller_position_unaligned() {
        let confines = Cube {
            min_position: V3c::unit(400.),
            size: 400.,
        };
        let update_size = 450;
        let y_offset_for_unalignment = 100;
        let execute_position = V3c::new(0, y_offset_for_unalignment, 0);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert!(target_child_sectant == 0 || target_child_sectant == 4);
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(
                    update_size_in_target.x,
                    update_size - confines.min_position.x as u32
                );
                assert!(
                    (update_size_in_target.y as f32 == target_bounds.size)
                        || update_size_in_target.y as f32
                            == ((update_size as f32 - confines.min_position.y
                                + y_offset_for_unalignment as f32)
                                % target_bounds.size)
                );
                assert_eq!(
                    update_size_in_target.z,
                    update_size - confines.min_position.z as u32
                );
            },
        );
    }

    #[test]
    fn test_sectant_execution_single_target_with_larger_position() {
        let confines = Cube {
            min_position: V3c::unit(400.),
            size: 400.,
        };
        let update_size = 100;
        let execute_position = V3c::new(0, 1000, 0);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |_position_in_target,
             _update_size_in_target,
             _target_child_sectant,
             &_target_bounds| {
                assert!(false);
            },
        );
    }

    #[test]
    fn test_sectant_execution_single_target_out_of_bounds() {
        let confines = Cube::root_bounds(400.);
        let update_size = 500;
        let execute_position = V3c::new(300, 300, 300);
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert_eq!(target_child_sectant, 63);
                assert_eq!(target_bounds.min_position, execute_position.into());
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(update_size_in_target.x as f32, target_bounds.size);
                assert_eq!(update_size_in_target.y as f32, target_bounds.size);
                assert_eq!(update_size_in_target.z as f32, target_bounds.size);
            },
        );
    }

    #[test]
    fn test_sectant_execution_aligned_target_within() {
        let confines = Cube::root_bounds(400.);
        let update_size = 400;
        let execute_position = V3c::new(100, 0, 0);
        let mut visited_sectants: Vec<u8> = vec![];
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                assert!(!visited_sectants.contains(&target_child_sectant));
                visited_sectants.push(target_child_sectant);
                if 1 == target_child_sectant {
                    assert_eq!(target_bounds.min_position, execute_position.into());
                }
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(update_size_in_target.x as f32, target_bounds.size);
                assert_eq!(update_size_in_target.y as f32, target_bounds.size);
                assert_eq!(update_size_in_target.z as f32, target_bounds.size);
            },
        );
        assert_eq!(
            visited_sectants.len(),
            (BOX_NODE_DIMENSION - 1) * BOX_NODE_DIMENSION * BOX_NODE_DIMENSION
        );
    }

    #[test]
    fn test_sectant_execution_aligned_target_out_of_bounds_smaller_position_larger_size() {
        let confines = Cube {
            min_position: V3c::unit(400.),
            size: 400.,
        };
        let update_size = 1000;
        let execute_position = V3c::new(500, 0, 0);
        let mut visited_sectants: Vec<u8> = vec![];
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                visited_sectants.push(target_child_sectant);
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(update_size_in_target.x as f32, target_bounds.size);
                assert_eq!(update_size_in_target.y as f32, target_bounds.size);
                assert_eq!(update_size_in_target.z as f32, target_bounds.size);
            },
        );
        assert_eq!(
            visited_sectants.len(),
            (BOX_NODE_DIMENSION - 1) * BOX_NODE_DIMENSION * BOX_NODE_DIMENSION,
            "visited sectant mismatch! \n visited sectants: {:?}",
            visited_sectants
        );
    }

    #[test]
    fn test_sectant_execution_aligned_target_out_of_bounds() {
        let confines = Cube::root_bounds(400.);
        let update_size = 500;
        let execute_position = V3c::new(100, 0, 0);
        let mut visited_sectants: Vec<u8> = vec![];
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                visited_sectants.push(target_child_sectant);
                if 1 == target_child_sectant {
                    assert_eq!(target_bounds.min_position, execute_position.into());
                }
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(update_size_in_target.x as f32, target_bounds.size);
                assert_eq!(update_size_in_target.y as f32, target_bounds.size);
                assert_eq!(update_size_in_target.z as f32, target_bounds.size);
            },
        );
        assert_eq!(
            visited_sectants.len(),
            (BOX_NODE_DIMENSION - 1) * BOX_NODE_DIMENSION * BOX_NODE_DIMENSION
        );
    }

    #[test]
    fn test_sectant_execution_unaligned_target_within() {
        let confines = Cube::root_bounds(400.);
        let update_size = 210;
        let execute_position = V3c::new(100, 0, 0);
        let mut visited_sectants: Vec<u8> = vec![];
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                visited_sectants.push(target_child_sectant);
                if 1 == target_child_sectant {
                    assert_eq!(target_bounds.min_position, execute_position.into());
                }
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert!(
                    update_size_in_target.x as f32 == target_bounds.size
                        || update_size_in_target.x as f32
                            == (update_size as f32 % target_bounds.size)
                );
                assert!(
                    update_size_in_target.y as f32 == target_bounds.size
                        || update_size_in_target.y as f32
                            == (update_size as f32 % target_bounds.size)
                );
                assert!(
                    update_size_in_target.z as f32 == target_bounds.size
                        || update_size_in_target.z as f32
                            == (update_size as f32 % target_bounds.size)
                );
            },
        );
        assert_eq!(visited_sectants.len(), (BOX_NODE_DIMENSION - 1).pow(3));
    }

    #[test]
    fn test_sectant_execution_unaligned_target_out_of_bounds() {
        let confines = Cube::root_bounds(400.);
        let update_size = 510;
        let execute_position = V3c::new(100, 0, 0);
        let mut visited_sectants: Vec<u8> = vec![];
        execute_for_relevant_sectants(
            &confines,
            &execute_position,
            update_size,
            |position_in_target, update_size_in_target, target_child_sectant, &target_bounds| {
                assert!(confines.contains(&V3c::from(
                    position_in_target + update_size_in_target - V3c::unit(1)
                )));
                visited_sectants.push(target_child_sectant);
                if 1 == target_child_sectant {
                    assert_eq!(target_bounds.min_position, execute_position.into());
                }
                assert_eq!(
                    target_bounds.size,
                    confines.size / BOX_NODE_DIMENSION as f32
                );
                assert_eq!(update_size_in_target.x as f32, target_bounds.size);
                assert_eq!(update_size_in_target.y as f32, target_bounds.size);
                assert_eq!(update_size_in_target.z as f32, target_bounds.size);
            },
        );
        assert_eq!(
            visited_sectants.len(),
            (BOX_NODE_DIMENSION - 1) * BOX_NODE_DIMENSION * BOX_NODE_DIMENSION
        );
    }
}

mod mipmap_tests {
    use crate::boxtree::{Albedo, BoxTree, MIPResamplingMethods, V3c, OOB_SECTANT};

    #[test]
    fn test_mixed_mip_lvl1() {
        let red: Albedo = 0xFF0000FF.into();
        let green: Albedo = 0x00FF00FF.into();
        let mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 2.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 2.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();

        let mut tree: BoxTree = BoxTree::new(4, 1).ok().unwrap();
        tree.auto_simplify = false;
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter);
        tree.insert(&V3c::new(0, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 1), &green)
            .expect("boxtree insert");

        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );
    }

    #[test]
    fn test_mixed_mip_lvl1_where_dim_is_32() {
        let red: Albedo = 0xFF0000FF.into();
        let green: Albedo = 0x00FF00FF.into();
        let mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 2.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 2.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();

        let mut tree: BoxTree = BoxTree::new(128, 32).ok().unwrap();
        tree.auto_simplify = false;
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter);
        tree.insert(&V3c::new(126, 126, 126), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(126, 126, 127), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(126, 127, 126), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(126, 127, 127), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(127, 126, 126), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(127, 126, 127), &green)
            .expect("boxtree insert");

        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(31, 31, 31))
            .albedo()
            .is_some());
        assert_eq!(
            mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(31, 31, 31))
                .albedo()
                .unwrap()
        );
    }

    #[test]
    fn test_simple_solid_mip_lvl2_where_dim_is_2() {
        let red: Albedo = 0xFF0000FF.into();

        let mut tree: BoxTree = BoxTree::new(8, 2).ok().unwrap();
        tree.auto_simplify = false;
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter);
        tree.insert(&V3c::new(0, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 0, 1), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 1), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 1), &red)
            .expect("boxtree insert");

        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            red,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 1, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 1, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 1, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 1, 1))
            .albedo()
            .is_none());
    }

    #[test]
    fn test_mixed_mip_lvl2_where_dim_is_2() {
        let red: Albedo = 0xFF0000FF.into();
        let green: Albedo = 0x00FF00FF.into();
        let mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 2.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 2.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();

        let mut tree: BoxTree = BoxTree::new(8, 2).ok().unwrap();
        tree.auto_simplify = false;
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter);
        tree.insert(&V3c::new(0, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 1), &green)
            .expect("boxtree insert");

        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 1, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 1, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 1))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 1, 0))
            .albedo()
            .is_none());
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 1, 1))
            .albedo()
            .is_none());
    }

    #[test]
    fn test_mixed_mip_lvl2_where_dim_is_4() {
        let red: Albedo = 0xFF0000FF.into();
        let green: Albedo = 0x00FF00FF.into();
        let blue: Albedo = 0x0000FFFF.into();

        let mut tree: BoxTree = BoxTree::new(64, 4).ok().unwrap();
        tree.auto_simplify = false;
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter)
            .set_method_at(2, MIPResamplingMethods::BoxFilter);
        tree.insert(&V3c::new(0, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 1), &green)
            .expect("boxtree insert");

        tree.insert(&V3c::new(16, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 1, 0), &blue)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(17, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(17, 0, 1), &blue)
            .expect("boxtree insert");

        // For child position 0,0,0
        let rg_mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 2.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 2.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(0, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rg_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(0, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // For child position 16,0,0
        let rgb_mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 3.).sqrt() as u32) << 8)
                | (((255_f32.powf(2.) / 3.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 3.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(1, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rgb_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(1, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // root mip position 0,0,0
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rg_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // root mip position 16,0,0
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rgb_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
                .albedo()
                .unwrap()
        );
    }

    #[test]
    fn test_mixed_mip_regeneration_lvl2_where_dim_is_4() {
        let red: Albedo = 0xFF0000FF.into();
        let green: Albedo = 0x00FF00FF.into();
        let blue: Albedo = 0x0000FFFF.into();

        let mut tree: BoxTree = BoxTree::new(64, 4).ok().unwrap();
        tree.auto_simplify = false;
        tree.insert(&V3c::new(0, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(0, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(1, 0, 1), &green)
            .expect("boxtree insert");

        tree.insert(&V3c::new(16, 0, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 0, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 1, 0), &blue)
            .expect("boxtree insert");
        tree.insert(&V3c::new(16, 1, 1), &green)
            .expect("boxtree insert");
        tree.insert(&V3c::new(17, 1, 0), &red)
            .expect("boxtree insert");
        tree.insert(&V3c::new(17, 0, 1), &blue)
            .expect("boxtree insert");

        // Switch MIP maps on, calculate the correct values
        tree.albedo_mip_map_resampling_strategy()
            .switch_albedo_mip_maps(true)
            .set_method_at(1, MIPResamplingMethods::BoxFilter)
            .set_method_at(2, MIPResamplingMethods::BoxFilter)
            .recalculate_mips();

        // For child position 0,0,0
        let rg_mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 2.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 2.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(0, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rg_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(0, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // For child position 8,0,0
        let rgb_mix: Albedo = (
            // Gamma corrected values follow mip = ((a^2 + b^2) / 2).sqrt()
            (((255_f32.powf(2.) / 3.).sqrt() as u32) << 8)
                | (((255_f32.powf(2.) / 3.).sqrt() as u32) << 16)
                | (((255_f32.powf(2.) / 3.).sqrt() as u32) << 24)
                | 0x000000FF
        )
        .into();
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(1, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rgb_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(1, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // root mip position 0,0,0
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rg_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(0, 0, 0))
                .albedo()
                .unwrap()
        );

        // root mip position 16,0,0
        assert!(tree
            .albedo_mip_map_resampling_strategy()
            .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
            .albedo()
            .is_some());
        assert_eq!(
            rgb_mix,
            *tree
                .albedo_mip_map_resampling_strategy()
                .sample_root_mip(OOB_SECTANT, &V3c::new(1, 0, 0))
                .albedo()
                .unwrap()
        );
    }
}
