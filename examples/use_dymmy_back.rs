use crate::common::dymmy_back::DummyGfxBack;
use crate::common::task_recv::TaskReceiver;
use f3_gfx::back::resource::mesh::{StaticMeshData, StaticMeshVertex, StaticMeshId};
use f3_gfx::data_src::{DataSource, TakeDataResult};
use f3_gfx::handler::{Getter, GfxHandler, TaskSender};
use f3_gfx::GfxBuilder;
use log::LevelFilter;
use std::sync::mpsc;
use f3_gfx::back::resource::task::add::AddResult;
use f3_gfx::res::GfxResource;
use f3_gfx::Gfx;

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
    gfx.run_tasks();

    assert!(mesh0.try_get().is_ok());
    assert!(mesh1.try_get().is_ok());
}

fn load_static_mesh(handler: &mut GfxHandler) -> Getter<AddResult<GfxResource<StaticMeshId>>> {
    handler.add_resource(Box::new(StaticMeshDataSrc{}))
}

#[derive(Debug)]
struct StaticMeshDataSrc {}

#[async_trait::async_trait]
impl DataSource<StaticMeshData> for StaticMeshDataSrc {
    async fn take_data(&mut self) -> TakeDataResult<StaticMeshData> {
        Ok(StaticMeshData {
            indices: vec![0, 1, 2],
            vertex_data: vec![StaticMeshVertex::default(); 3],
        })
    }
}
