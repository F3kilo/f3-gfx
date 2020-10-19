use crate::back::{Backend, GeomId, RenderError, TexId};
use crate::res::Resource;
use crate::scene::Scene;
use crate::task_counter::TaskCounter;
use crate::LoadResult;
use std::path::PathBuf;

use std::sync::mpsc::{Receiver, Sender};

use tokio::runtime::Runtime;

pub struct Gfx {
    back: Box<dyn Backend>,
    task_counter: TaskCounter,
    rt: Runtime,
}

impl Gfx {
    pub fn new(back: Box<dyn Backend>) -> Self {
        let task_counter = TaskCounter::new();
        let rt = Runtime::new().unwrap();
        Self {
            back,
            task_counter,
            rt,
        }
    }

    pub fn replace_back(&mut self, back: Box<dyn Backend>) {
        self.task_counter.wait_all_ready();
        todo!("reload all data to new back")
    }

    pub fn load_tex(&mut self, path: PathBuf, tx: Sender<Tex>) {
        log::trace!("Start load tex: {:?}", path);
        self.task_counter.inc();
    }

    // pub fn load_geom(&self, path: PathBuf) -> ReceiveOnce<GeomReceiver> {
    //     log::trace!("Start load geom: {:?}", path);
    //     let (tx, rx) = mpsc::channel();
    //     let _ = self.task_tx.send(Box::new(LoadGeom::new(path, tx)));
    //     ReceiveOnce::new(rx)
    // }
    //
    // pub fn render(&self, scene: Scene, render_info: RenderInfo) -> ReceiveOnce<RenderReceiver> {
    //     log::trace!("Start render");
    //     let (tx, rx) = mpsc::channel();
    //     let render_task = Box::new(Render::new(scene, render_info, tx));
    //     let _ = self.task_tx.send(render_task);
    //     ReceiveOnce::new(rx)
    // }
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;

pub type Geom = Resource<GeomId>;
type GeomReceiver = Receiver<LoadResult<Geom>>;

pub type RenderResult = Result<Tex, RenderError>;
pub type RenderReceiver = Receiver<(RenderResult, Scene)>;
