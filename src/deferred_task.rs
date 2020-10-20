use crate::back::{GeomId, TexId};
use crate::gfx::{Geom, Tex};
use crate::waiter::Setter;
use crate::LoadResult;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug)]
pub enum DeferredTask {
    LoadTex(PathBuf, Setter<LoadResult<Tex>>),
    RemoveTex(TexId),
    LoadGeom(PathBuf, Setter<LoadResult<Geom>>),
    RemoveGeom(GeomId),
}

pub struct DeferredTaskStorage {
    tx: Sender<DeferredTask>,
    rx: Receiver<DeferredTask>,
}

impl DeferredTaskStorage {
    pub fn pusher(&self) -> TaskPusher {
        TaskPusher(Arc::new(Mutex::new(self.tx.clone())))
    }

    pub fn next(&self) -> Option<DeferredTask> {
        self.rx.try_recv().ok()
    }
}

impl Default for DeferredTaskStorage {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

#[derive(Clone)]
pub struct TaskPusher(Arc<Mutex<Sender<DeferredTask>>>);

impl TaskPusher {
    pub fn push(&self, task: DeferredTask) {
        self.0
            .lock()
            .expect("DeferredTaskPusher sender mutex is poisoned")
            .send(task)
            .expect("DeferredTaskPusher sender can't send task");
    }
}
