use crate::back::resource::mesh::StaticMeshId;
use crate::res::GfxResource;

/// Struct that represent everything that must be rendered.
#[derive(Debug, Default, Clone)]
pub struct Scene {
    pub world_transforms: RawMat4,
    pub color_static_mesh: Vec<ColorStaticMesh>,
}

/// Represent colored static.
#[derive(Debug, Clone)]
pub struct ColorStaticMesh {
    pub instance: InstanceData,
    pub mesh: GfxResource<StaticMeshId>,
}

pub type RawMat4 = [f32; 16];

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InstanceData {
    pub transforms: RawMat4,
}
