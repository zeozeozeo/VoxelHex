use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn icon() -> UiLayout {
    UiLayout::window().pos(Ab(4.)).width(24.).height(24.).pack()
}

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
        .with_children(|ui_root| {
            // Model and properties
            ui_root
                .spawn((UiLayout::solid().align_x(-1.).align_y(-1.)).pack())
                .with_children(|ui_model_panel| {
                    // model name panel
                    ui_model_panel
                        .spawn((
                            crate::components::Model,
                            crate::components::Container,
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 5.)))
                                .size(Ab((768.0, 32.0)))
                                .pack(),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                        ))
                        .with_children(|ui_modelname_field| {
                            ui_modelname_field.spawn((
                                icon(),
                                Sprite::from_image(asset_server.load("ui/open_icon.png")),
                            ));
                            ui_modelname_field.spawn((
                                crate::components::Model,
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
                    ui_model_panel
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_resolution_panel| {
                            ui_resolution_panel.spawn((
                                crate::components::Output,
                                crate::components::Link,
                                Button,
                                icon(),
                                UiColor::from(Color::srgb(1., 1., 1.)),
                                Sprite::from_image(asset_server.load("ui/linked_icon.png")),
                            ));

                            ui_resolution_panel.spawn((
                                crate::components::Output,
                                crate::components::Width,
                                Button,
                                crate::components::UiAction {
                                    is_active: false,
                                    change_sensitivity: 0.05,
                                    boundaries: [128, 7680],
                                },
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((35., 8.)))
                                    .size(Ab((55.0, 18.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(0.62, 0.1, 0.4, 0.7),
                                    Vec2::new(1., 1.),
                                ),
                                Text2d::new("1920"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
                            ui_resolution_panel.spawn((
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((100., 7.)))
                                    .size(Ab((15.0, 18.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("x"),
                            ));

                            ui_resolution_panel.spawn((
                                crate::components::Output,
                                crate::components::Height,
                                Button,
                                crate::components::UiAction {
                                    is_active: false,
                                    change_sensitivity: 0.05,
                                    boundaries: [72, 4320],
                                },
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((125., 8.)))
                                    .size(Ab((55.0, 20.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(0.62, 0.1, 0.4, 0.7),
                                    Vec2::new(1., 1.),
                                ),
                                Text2d::new("1080"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
                        });

                    // Performance panel
                    ui_model_panel
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((210., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_performance_panel| {
                            ui_performance_panel.spawn((
                                crate::components::Performance,
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
                    ui_model_panel
                        .spawn((
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((415., 40.)))
                                .size(Ab((200.0, 32.0)))
                                .pack(),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                        ))
                        .with_children(|ui_versions_panel| {
                            ui_versions_panel.spawn((
                                crate::components::Model,
                                crate::components::Info,
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
                    ui_model_panel.spawn((
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((620., 45.)))
                            .size(Ab((150.0, 25.0)))
                            .pack(),
                        UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                        Text2d::new("Press 'G' to hide UI"),
                    ));

                    // Loading bar
                    ui_model_panel
                        .spawn((
                            crate::components::Model,
                            crate::components::Loading,
                            crate::components::Slider,
                            crate::components::Container,
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((5., 75.)))
                                .size(Ab((768.0, 25.0)))
                                .pack(),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            UiColor::from(Color::srgb(0.3, 0.0, 0.23)),
                        ))
                        .with_children(|ui_loading_bar| {
                            ui_loading_bar.spawn((
                                crate::components::Model,
                                crate::components::Loading,
                                crate::components::Slider,
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((0., 0.)))
                                    .size(Ab((400.0, 25.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(1., 1., 1., 0.7),
                                    Vec2::new(1., 1.),
                                ),
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
                                        crate::components::Model,
                                        crate::components::Loading,
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
                                        crate::components::Model,
                                        crate::components::Status,
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
                    crate::components::Camera,
                    crate::components::Container,
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(5.))
                        .size(Ab((400.0, 100.0)))
                        .pack(),
                    Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
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
                            crate::components::Camera,
                            crate::components::Depth,
                            crate::components::Slider,
                            crate::components::Container,
                            crate::components::UiAction {
                                is_active: false,
                                change_sensitivity: 0.1,
                                boundaries: [1, 100],
                            },
                            UiLayout::window()
                                .anchor(Anchor::TopLeft)
                                .pos(Ab((105., 5.)))
                                .width(Rl(100.) - Ab(105.))
                                .height(Ab(25.))
                                .pack(),
                            Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                            UiColor::from(Color::srgb(0.3, 0.0, 0.23)),
                            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                        ))
                        .with_children(|ui_fov_panel| {
                            ui_fov_panel.spawn((
                                crate::components::Camera,
                                crate::components::Depth,
                                crate::components::Slider,
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((0., 0.)))
                                    .size(Rl((50.0, 100.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(1., 1., 1., 0.7),
                                    Vec2::new(1., 1.),
                                ),
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
                                crate::components::Camera,
                                crate::components::Link,
                                Button,
                                icon(),
                                UiColor::from(Color::srgb(1., 1., 1.)),
                                Sprite::from_image(asset_server.load("ui/linked_icon.png")),
                            ));

                            ui_camera_intrinsics_viewport_row.spawn((
                                crate::components::Camera,
                                crate::components::Width,
                                Button,
                                crate::components::UiAction {
                                    is_active: false,
                                    change_sensitivity: 0.05,
                                    boundaries: [5, 250],
                                },
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((40., 3.)))
                                    .size(Ab((45.0, 18.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(0.62, 0.1, 0.4, 0.7),
                                    Vec2::new(1., 1.),
                                ),
                                Text2d::new("100"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));

                            ui_camera_intrinsics_viewport_row.spawn((
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((100., 0.)))
                                    .size(Ab((15.0, 25.0)))
                                    .pack(),
                                UiColor::from(Color::srgb(0.88, 0.62, 0.49)),
                                Text2d::new("x"),
                            ));

                            ui_camera_intrinsics_viewport_row.spawn((
                                crate::components::Camera,
                                crate::components::Height,
                                Button,
                                crate::components::UiAction {
                                    is_active: false,
                                    change_sensitivity: 0.05,
                                    boundaries: [5, 250],
                                },
                                UiLayout::window()
                                    .anchor(Anchor::TopLeft)
                                    .pos(Ab((130., 3.)))
                                    .size(Ab((45.0, 18.0)))
                                    .pack(),
                                Sprite::from_color(
                                    Color::srgba(0.62, 0.1, 0.4, 0.7),
                                    Vec2::new(1., 1.),
                                ),
                                Text2d::new("100"),
                                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                            ));
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

                    ui_camera_intrinsics_panel.spawn((
                        crate::components::Camera,
                        crate::components::Depth,
                        Button,
                        crate::components::UiAction {
                            is_active: false,
                            change_sensitivity: 0.05,
                            boundaries: [10, 500000],
                        },
                        UiLayout::window()
                            .anchor(Anchor::TopLeft)
                            .pos(Ab((105., 65.)))
                            .size(Ab((55.0, 18.0)))
                            .pack(),
                        Sprite::from_color(Color::srgba(0.62, 0.1, 0.4, 0.7), Vec2::new(1., 1.)),
                        Text2d::new("1024"),
                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                    ));
                });

            // Shortcuts button
            ui_root
                .spawn((
                    Visibility::Hidden,
                    crate::components::Info,
                    crate::components::Container,
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(200.))
                        .size(Ab((32.0, 32.0)))
                        .pack(),
                    Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                    UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                ))
                .with_children(|ui_shortcuts_panel| {
                    ui_shortcuts_panel.spawn((
                        crate::components::Info,
                        Button,
                        icon(),
                        Sprite::from_image(asset_server.load("ui/info_active.png")),
                    ));
                });

            // Shortcuts panel
            ui_root
                .spawn((
                    Visibility::Visible,
                    crate::components::Info,
                    crate::components::Container,
                    crate::components::Expanded,
                    UiLayout::window()
                        .anchor(Anchor::TopRight)
                        .x(Rl(100.) - Ab(5.))
                        .y(Ab(200.))
                        .size(Ab((200.0, 512.0)))
                        .pack(),
                    Sprite::from_color(Color::srgba(1., 1., 1., 0.7), Vec2::new(1., 1.)),
                    UiColor::from(Color::srgb(0.2, 0.1, 0.25)),
                ))
                .with_children(|ui_shortcuts_panel| {
                    ui_shortcuts_panel.spawn((
                        crate::components::Info,
                        Button,
                        crate::components::Expanded,
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
