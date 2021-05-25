use crate::scene::{Scene, RawMat4};
use std::sync::Arc;
use crate::res::GfxResource;
use crate::back::resource::window::WindowId;

/// Task for presenting scene on screeen.
#[derive(Debug)]
pub struct PresentTask {
    present_info: PresentInfo,
    scene: Arc<Scene>,
}

impl PresentTask {
    /// Creates new present task.
    pub fn new(
        present_info: PresentInfo,
        scene: Arc<Scene>,
    ) -> Self {
        Self {
            present_info,
            scene,
        }
    }

    /// Splits present task to present info, scene and result sender.
    pub fn into_inner(self) -> (PresentInfo, Arc<Scene>) {
        (self.present_info, self.scene)
    }
}

/// Information for presenting.
#[derive(Debug, Clone)]
pub struct PresentInfo {
    pub render_info: RenderProps,
    pub window: GfxResource<WindowId>,
}

/// Rendering properties.
#[derive(Debug, Clone, Default)]
pub struct RenderProps {
    pub camera: CameraInfo
}

/// Information about camera.
#[derive(Debug, Copy, Clone, Default)]
pub struct CameraInfo {
    pub view: RawMat4,
    pub proj: RawMat4,
}
