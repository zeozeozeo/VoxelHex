pub mod cpu;
mod tests;

#[cfg(feature = "bevy_wgpu")]
pub mod bevy;

pub use crate::spatial::raytracing::Ray;

#[cfg(feature = "bevy_wgpu")]
pub use bevy::types::{
    BoxTreeGPUHost, BoxTreeGPUView, BoxTreeRenderData, BoxTreeSpyGlass, RenderBevyPlugin,
    VhxViewSet, Viewport,
};
