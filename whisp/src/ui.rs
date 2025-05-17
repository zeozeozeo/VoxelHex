use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn panel_sprite() -> Sprite {
    Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.))
}

fn icon() -> UiLayout {
    UiLayout::window().pos(Ab(4.)).width(24.).height(24.).pack()
}

pub(crate) fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
        .with_children(|ui_root| {
            // Model and properties
            ui_root
                .spawn((UiLayout::solid().align_x(-1.).align_y(-1.)).pack())
                .with_children(|ui_top_menu| {
                    // model name panel
                    ui_top_menu
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 5.)))
                                .size(Ab((768.0, 32.0)))
                                .pack(),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                            panel_sprite(),
                            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                        ))
                        .with_children(|ui_modelname_field| {
                            ui_modelname_field.spawn((
                                icon(),
                                Sprite::from_image(asset_server.load("ui/open_icon.png")),
                            ));
                            ui_modelname_field.spawn((
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((32., 4.)))
                                    .width(Rl(100.) - Ab(37.))
                                    .height(Ab(25.))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("ginerbread_house_by_kirra_luan(1024^3)"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
                        });

                    // resolution panel
                    ui_top_menu
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_resolution_panel| {
                            ui_resolution_panel.spawn((
                                icon(),
                                UiColor::from(Color::srgb(1., 1., 1.)),
                                Sprite::from_image(asset_server.load("ui/linked_icon.png")),
                            ));

                            ui_resolution_panel
                                .spawn((
                                    Name::new("outputResolutionWidthBtn"),
                                    UiLayout::window()
                                        .anchor(Anchor::TopLeft)
                                        .pos(Ab((35., 8.)))
                                        .size(Ab((55.0, 25.0)))
                                        .pack(),
                                    panel_sprite(),
                                    UiColor::from(Color::srgb(0.62, 0.1, 0.4)),
                                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                ))
                                .with_children(|ui_output_resolution_width| {
                                    ui_output_resolution_width.spawn((
                                        Name::new("outputResolutionWidthText"),
                                        UiLayout::window()
                                            .anchor(Anchor::TopLeft)
                                            .pos(Ab((5., 2.)))
                                            .size(Ab((45.0, 15.0)))
                                            .pack(),
                                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                        Text2d::new("1920"),
                                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                    ));
                                });
                            ui_resolution_panel.spawn((
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((100., 2.)))
                                    .size(Ab((15.0, 20.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("x"),
                            ));

                            ui_resolution_panel
                                .spawn((
                                    Name::new("outputResolutionHeightBtn"),
                                    UiLayout::window()
                                        .anchor(Anchor::TopLeft)
                                        .pos(Ab((125., 8.)))
                                        .size(Ab((55.0, 25.0)))
                                        .pack(),
                                    panel_sprite(),
                                    UiColor::from(Color::srgb(0.62, 0.1, 0.4)),
                                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                ))
                                .with_children(|ui_output_resolution_height| {
                                    ui_output_resolution_height.spawn((
                                        Name::new("outputResolutionHeightText"),
                                        UiLayout::window()
                                            .anchor(Anchor::TopLeft)
                                            .pos(Ab((5., 2.)))
                                            .size(Ab((45.0, 15.0)))
                                            .pack(),
                                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                        Text2d::new("1080"),
                                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                    ));
                                });
                        });

                    // Performance panel
                    ui_top_menu
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((210., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_performance_panel| {
                            ui_performance_panel.spawn((
                                Name::new("perofrmanceText"),
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((25., 5.)))
                                    .size(Ab((150.0, 30.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("120fps / 5ms"),
                            ));
                        });

                    // versions panel
                    ui_top_menu
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((415., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_performance_panel| {
                            ui_performance_panel.spawn((
                                Name::new("versionsText"),
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((15., 5.)))
                                    .size(Ab((165.0, 20.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("model: v0.1  / app: v0.1"),
                            ));
                        });

                    // G to hide text
                    ui_top_menu.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((620., 45.)))
                            .size(Ab((150.0, 25.0)))
                            .pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        Text2d::new("Press 'G' to hide UI"),
                    ));

                    // Loading bar
                    ui_top_menu
                        .spawn((
                            Name::new("loadingPanel"),
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 75.)))
                                .size(Ab((768.0, 25.0)))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.3, 0.0, 0.23)),
                        ))
                        .with_children(|ui_loading_bar| {
                            ui_loading_bar.spawn((
                                Name::new("loadingPanelBar"),
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((0., 0.)))
                                    .size(Ab((400.0, 25.0)))
                                    .pack(),
                                panel_sprite(),
                                UiColor::from(Color::srgb(0.96, 0.0, 0.72)),
                            ));

                            ui_loading_bar
                                .spawn(
                                    UiLayout::window()
                                        .pos(Ab(2.))
                                        .size(Rl(100.) - Ab(2.))
                                        .anchor(Anchor::TopLeft)
                                        .pack(),
                                )
                                .with_children(|ui_loading_bar_padding| {
                                    ui_loading_bar_padding.spawn((
                                        Name::new("loadingText"),
                                        UiLayout::solid()
                                            .scaling(Scaling::VerFill)
                                            .align_x(-1.)
                                            .size((400.0, 20.0))
                                            .pack(),
                                        UiColor::from(Color::srgb(0., 0., 0.)),
                                        UiTextSize::from(Rh(95.0)),
                                        Text2d::new("Loading.."),
                                    ));

                                    ui_loading_bar_padding.spawn((
                                        Name::new("loadedText"),
                                        UiLayout::solid()
                                            .scaling(Scaling::VerFill)
                                            .align_x(1.)
                                            .size((400.0, 20.0))
                                            .pack(),
                                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                        UiTextSize::from(Rh(95.0)),
                                        Text2d::new("Loaded! Here is a message"),
                                    ));
                                });
                        });
                });

            // Camera Intrinsics panel
            ui_root
                .spawn((
                    Name::new("cameraIntrinicsPanel"),
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(5.))
                        .size(Ab((400.0, 150.0)))
                        .pack(),
                    panel_sprite(),
                    UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                ))
                .with_children(|ui_camera_intrinsics_panel| {
                    ui_camera_intrinsics_panel.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((5., 5.)))
                            .size(Ab((100., 25.)))
                            .pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        UiTextSize::from(Ab(15.0)),
                        Text2d::new("FOV:"),
                    ));

                    ui_camera_intrinsics_panel
                        .spawn((
                            Name::new("fovPanel"),
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((105., 5.)))
                                .width(Rl(100.) - Ab(105.))
                                .height(Ab(25.))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.3, 0.0, 0.23)),
                            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                        ))
                        .with_children(|ui_fov_panel| {
                            ui_fov_panel.spawn((
                                Name::new("fovPanelBar"),
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((0., 0.)))
                                    .size(Rl((50.0, 100.0)))
                                    .pack(),
                                panel_sprite(),
                                UiColor::from(Color::srgb(0.96, 0.0, 0.72)),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
                        });

                    ui_camera_intrinsics_panel.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((5., 35.)))
                            .size(Ab((100., 25.)))
                            .pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        UiTextSize::from(Ab(15.0)),
                        Text2d::new("Viewport:"),
                    ));

                    ui_camera_intrinsics_panel
                        .spawn(
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((105., 35.)))
                                .width(Rl(100.) - Ab(100.))
                                .height(Ab(25.))
                                .pack(),
                        )
                        .with_children(|ui_camera_intrinsics_viewport_row| {
                            ui_camera_intrinsics_viewport_row.spawn((
                                icon(),
                                UiColor::from(Color::srgb(1., 1., 1.)),
                                Sprite::from_image(asset_server.load("ui/linked_icon.png")),
                            ));

                            ui_camera_intrinsics_viewport_row
                                .spawn((
                                    Name::new("viewportWidthBtn"),
                                    UiLayout::window()
                                        .anchor(Anchor::TopLeft)
                                        .pos(Ab((35., 0.)))
                                        .size(Ab((55.0, 25.0)))
                                        .pack(),
                                    panel_sprite(),
                                    UiColor::from(Color::srgb(0.62, 0.1, 0.4)),
                                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                ))
                                .with_children(|ui_output_resolution_width| {
                                    ui_output_resolution_width.spawn((
                                        Name::new("viewportWidthText"),
                                        UiLayout::window()
                                            .anchor(Anchor::TopLeft)
                                            .pos(Ab((5., 0.)))
                                            .size(Ab((45.0, 25.0)))
                                            .pack(),
                                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                        Text2d::new("100"),
                                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                    ));
                                });
                            ui_camera_intrinsics_viewport_row.spawn((
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((100., 2.)))
                                    .size(Ab((15.0, 25.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("x"),
                            ));

                            ui_camera_intrinsics_viewport_row
                                .spawn((
                                    Name::new("viewportHeightBtn"),
                                    UiLayout::window()
                                        .anchor(Anchor::TopLeft)
                                        .pos(Ab((125., 0.)))
                                        .size(Ab((55.0, 25.0)))
                                        .pack(),
                                    panel_sprite(),
                                    UiColor::from(Color::srgb(0.62, 0.1, 0.4)),
                                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                ))
                                .with_children(|ui_output_resolution_height| {
                                    ui_output_resolution_height.spawn((
                                        Name::new("viewportHeightText"),
                                        UiLayout::window()
                                            .anchor(Anchor::TopLeft)
                                            .pos(Ab((5., 0.)))
                                            .size(Ab((45.0, 25.0)))
                                            .pack(),
                                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                        Text2d::new("100"),
                                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                                    ));
                                });
                        });

                    ui_camera_intrinsics_panel.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((5., 65.)))
                            .size(Ab((100., 25.)))
                            .pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        UiTextSize::from(Ab(15.0)),
                        Text2d::new("View dist.:"),
                    ));

                    ui_camera_intrinsics_panel
                        .spawn((
                            Name::new("viewDistanceBtn"),
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((105., 65.)))
                                .size(Ab((55.0, 25.0)))
                                .pack(),
                            panel_sprite(),
                            UiColor::from(Color::srgb(0.62, 0.1, 0.4)),
                            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                        ))
                        .with_children(|ui_output_resolution_height| {
                            ui_output_resolution_height.spawn((
                                Name::new("viewDistanceText"),
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((5., 0.)))
                                    .size(Ab((45.0, 25.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("1024"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
                        });
                });

            // Shortcuts button
            ui_root
                .spawn((
                    Visibility::Hidden,
                    Name::new("shortcutsInfoButton"),
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(200.))
                        .size(Ab((32.0, 32.0)))
                        .pack(),
                    panel_sprite(),
                    UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                ))
                .with_children(|ui_shortcuts_panel| {
                    ui_shortcuts_panel.spawn((
                        icon(),
                        Sprite::from_image(asset_server.load("ui/info_active.png")),
                    ));
                });
            // .observe(
            //     |_: Trigger<Pointer<Click>>, mut shortcuts_panel: Query<(Name, )>| {
            //         // Close the app on click
            //         exit.send(AppExit::Success);
            //     },
            // );

            // Shortcuts panel
            ui_root
                .spawn((
                    Name::new("shortcutsInfoPanel"),
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(200.))
                        .size(Ab((200.0, 512.0)))
                        .pack(),
                    panel_sprite(),
                    UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                ))
                .with_children(|ui_shortcuts_panel| {
                    ui_shortcuts_panel.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopRight)
                            .size(Ab((24., 24.)))
                            .x(Rl(100.) - Ab(4.))
                            .y(Ab(4.))
                            .pack(),
                        Sprite::from_image(asset_server.load("ui/info_active.png")),
                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                    ));

                    ui_shortcuts_panel.spawn((
                        Name::new("loadedText"),
                        UiLayout::solid().scaling(Scaling::Fit).align_x(1.).pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        Text2d::new(
                            "
G - hide UI
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
Mouse Click - clikity
",
                        ),
                    ));
                });

            // render texture
            ui_root.spawn((
                UiLayout::solid()
                    .size((2., 1.))
                    .align_x(0.)
                    .align_y(0.)
                    .pack(),
                Sprite::from_color(Color::srgb(0., 0., 0.), Vec2::new(1920.0, 1080.0)),
            ));
        });
}
