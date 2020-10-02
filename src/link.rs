use crate::back::{Backend, TexId};
use crate::res::Resource;
use crate::task::load_tex::LoadTex;
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
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;
