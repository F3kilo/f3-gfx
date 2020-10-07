use crate::back::{Backend, GeomId, RenderError, RenderInfo, TexId};
use crate::res::Resource;
use crate::scene::Scene;
use crate::task::load_geom::LoadGeom;
use crate::task::load_tex::LoadTex;
use crate::task::render::Render;
use crate::task::Task;
use crate::waiter::ReceiveOnce;
use crate::LoadResult;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
pub struct Link {
    task_tx: Sender<Box<dyn Task>>,
}

impl Link {
    pub fn new(task_tx: Sender<Box<dyn Task>>) -> Self {
        Self { task_tx }
    }

    pub fn replace_back(&self, back: Box<dyn Backend>) {
        todo!()
    }

    pub fn load_tex(&self, path: PathBuf) -> ReceiveOnce<TexReceiver> {
        log::trace!("Start load tex: {:?}", path);
        let (tx, rx) = mpsc::channel();
        let _ = self.task_tx.send(Box::new(LoadTex::new(path, tx)));
        ReceiveOnce::new(rx)
    }

    pub fn load_geom(&self, path: PathBuf) -> ReceiveOnce<GeomReceiver> {
        log::trace!("Start load tex: {:?}", path);
        let (tx, rx) = mpsc::channel();
        let _ = self.task_tx.send(Box::new(LoadGeom::new(path, tx)));
        ReceiveOnce::new(rx)
    }

    pub fn render(&self, scene: Scene, render_info: RenderInfo) -> ReceiveOnce<RenderReceiver> {
        log::trace!("Start render");
        let (tx, rx) = mpsc::channel();
        let render_task = Box::new(Render::new(scene, render_info, tx));
        let _ = self.task_tx.send(render_task);
        ReceiveOnce::new(rx)
    }
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;

pub type Geom = Resource<GeomId>;
type GeomReceiver = Receiver<LoadResult<Geom>>;

pub type RenderResult = Result<Tex, RenderError>;
pub type RenderReceiver = Receiver<(RenderResult, Scene)>;
