use crate::common::dymmy_back::DummyGfxBack;
use f3_gfx::back::present::PresentInfo;
use f3_gfx::back::resource::mesh::{StaticMeshData, StaticMeshId, StaticMeshVertex};
use f3_gfx::handler::GfxHandler;
use f3_gfx::res::set_get::Getter;
use f3_gfx::res::GfxResource;
use f3_gfx::scene::{ColorStaticMesh, InstanceData, Scene};
use f3_gfx::Gfx;
use sloggers::Build;
use std::sync::Arc;
use sloggers::types::SourceLocation;

mod common;

fn main() {
    let logger = sloggers::terminal::TerminalLoggerBuilder::new()
        .level(sloggers::types::Severity::Trace)
        .source_location(SourceLocation::FileAndLine)
        .build()
        .unwrap();

    let back = Box::new(DummyGfxBack::new(logger.clone()));
    let mut gfx = Gfx::new(back, Some(logger));
    let mut handler = gfx.create_handler();
    let mut mesh0 = load_static_mesh(&mut handler);
    let mut mesh1 = load_static_mesh(&mut handler);
    assert!(mesh0.get().is_err());
    assert!(mesh1.get().is_err());
    gfx.update().unwrap();
    let mesh0 = mesh0.get().unwrap();
    let mesh1 = mesh1.get().unwrap();

    let present_info = PresentInfo::default();
    let mut scene = Scene::default();
    scene.color_static_mesh.push(ColorStaticMesh {
        mesh: mesh0,
        instance: InstanceData::default(),
    });

    scene.color_static_mesh.push(ColorStaticMesh {
        mesh: mesh1,
        instance: InstanceData::default(),
    });

    handler.present_scene(present_info, Arc::new(scene));
    gfx.update().unwrap();
}

fn load_static_mesh(handler: &mut GfxHandler) -> Getter<GfxResource<StaticMeshId>> {
    handler.add_resource(static_mesh_data())
}

fn static_mesh_data() -> StaticMeshData {
    StaticMeshData {
        indices: vec![0, 1, 2],
        vertices: vec![StaticMeshVertex::default(); 3],
    }
}
