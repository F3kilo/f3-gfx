use crate::back::ResultSetter;
use crate::scene::{Scene, RawMat4};

/// Task for presenting scene on screeen.
#[derive(Debug)]
pub struct PresentTask {
    present_info: PresentInfo,
    scene: Scene,
    result_setter: Box<dyn ResultSetter<Scene>>,
}

impl PresentTask {
    /// Creates new present task.
    pub fn new(
        present_info: PresentInfo,
        scene: Scene,
        result_setter: Box<dyn ResultSetter<Scene>>,
    ) -> Self {
        Self {
            present_info,
            result_setter,
            scene,
        }
    }

    /// Splits present task to present info, scene and result sender.
    pub fn into_inner(self) -> (PresentInfo, Scene, Box<dyn ResultSetter<Scene>>) {
        (self.present_info, self.scene, self.result_setter)
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
