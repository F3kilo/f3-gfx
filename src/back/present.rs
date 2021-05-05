use crate::scene::{Scene, RawMat4};
use std::sync::Arc;

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
#[derive(Debug, Copy, Clone, Default)]
pub struct PresentInfo {
    camera: CameraInfo
}

/// Information about camera.
#[derive(Debug, Copy, Clone, Default)]
pub struct CameraInfo {
    pub view: RawMat4,
    pub proj: RawMat4,
}
