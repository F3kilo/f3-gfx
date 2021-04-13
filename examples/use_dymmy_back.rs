use crate::common::dymmy_back::DummyGfxBack;
use crate::common::task_recv::TaskReceiver;
use f3_gfx::back::present::PresentInfo;
use f3_gfx::back::resource::mesh::{StaticMeshData, StaticMeshId, StaticMeshVertex};
use f3_gfx::back::resource::task::add::AddResult;
use f3_gfx::handler::{GetError, Getter, GfxHandler, TaskSender};
use f3_gfx::res::GfxResource;
use f3_gfx::scene::{ColorStaticMesh, InstanceData, Scene};
use f3_gfx::Gfx;
use f3_gfx::GfxBuilder;
use log::LevelFilter;
use std::sync::mpsc;

mod common;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();

    let back = DummyGfxBack::default();
    let (tx, rx) = mpsc::channel();
    let task_receiver = TaskReceiver::new(rx);
    let mut gfx = GfxBuilder::new(task_receiver, Box::new(back)).build();
    let task_sender = TaskSender::new(tx);
    let mut handler = GfxHandler::new(task_sender);
    let mut mesh0 = load_static_mesh(&mut handler);
    let mut mesh1 = load_static_mesh(&mut handler);
    assert!(mesh0.try_get().is_err());
    assert!(mesh1.try_get().is_err());
    gfx.run_tasks().unwrap();
    gfx.update().unwrap();
    let mesh0 = mesh0.try_get().unwrap().unwrap();
    let mesh1 = mesh1.try_get().unwrap().unwrap();

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

    let mut present_result = handler.present_scene(present_info, scene);
    assert!(!present_result.try_get().is_ok());
    gfx.run_tasks().unwrap();
    gfx.update().unwrap();
    assert!(present_result.try_get().is_ok());
    assert!(matches!(
        present_result.try_get(),
        Err(GetError::AlreadyTaken)
    ))
}

fn load_static_mesh(handler: &mut GfxHandler) -> Getter<AddResult<GfxResource<StaticMeshId>>> {
    handler.add_resource(static_mesh_data())
}

fn static_mesh_data() -> StaticMeshData {
    StaticMeshData {
        indices: vec![0, 1, 2],
        vertices: vec![StaticMeshVertex::default(); 3],
    }
}
