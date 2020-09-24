use crate::back::Backend;
use crate::task::load_tex::LoadTex;
use crate::task::Task;
use crate::tex_waiter::{TexRemover, TexWaiter};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

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

    pub fn load_tex(&self, path: PathBuf) -> TexWaiter {
        let (tx, rx) = mpsc::channel();
        let _ = self.task_tx.send(Box::new(LoadTex::new(path, tx)));
        TexWaiter::new(rx, TexRemover::new(self.task_tx.clone()))
    }
}
