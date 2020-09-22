use crate::back::{Backend, LoadResult, TexId};
use crate::tex::Tex;
use crate::tex_waiter::{TexUnloader, TexWaiter};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc};
use std::{mem, thread};
use tokio::runtime::Runtime;
use tokio::time::Duration;

pub fn run(back: Box<dyn Backend>) -> Link {
    let (task_tx, task_rx) = mpsc::channel();
    let link = Link::new(task_tx.clone());

    thread::spawn(move || {
        let mut tasker = Tasker::new(back, task_tx, task_rx);
        while tasker.start_next_task() {}
    });

    link
}

#[derive(Clone)]
pub struct Link {
    task_tx: Sender<Task>,
}

impl Link {
    fn new(task_tx: Sender<Task>) -> Self {
        Self { task_tx }
    }

    pub fn replace_back(&self, back: Box<dyn Backend>) {
        let _ = self.task_tx.send(Task::ReplaceBack(back));
    }

    pub fn load_tex(&self, path: PathBuf) -> TexWaiter {
        let (tx, rx) = mpsc::channel();

        TexWaiter::new(rx, TexUnloader::new(self.task_tx.clone()))
    }
}

pub enum Task {
    ReplaceBack(Box<dyn Backend>),
    LoadTex(LoadTexTask),
    UnloadTex(TexId),
    LaterUnloadTex(Receiver<LoadResult<TexId>>),
}

struct Tasker {
    back: Box<dyn Backend>,
    task_tx: Sender<Task>,
    task_rx: Receiver<Task>,
    rt: Arc<Runtime>,
}

impl Tasker {
    fn new(back: Box<dyn Backend>, task_tx: Sender<Task>, task_rx: Receiver<Task>) -> Self {
        Self {
            back,
            task_tx,
            task_rx,
            rt: Arc::new(Runtime::new().expect("Can't run tokio Runtime")),
        }
    }

    pub fn start_next_task(&mut self) -> bool {
        match self.task_rx.recv() {
            Ok(t) => {
                self.start(t);
                true
            }
            Err(_) => false,
        }
    }

    pub fn start(&mut self, task: Task) {
        match task {
            Task::ReplaceBack(b) => self.replace_back(b),
            Task::LoadTex(task) => self.load_tex(task),
            Task::UnloadTex(id) => self.back.unload_tex(id),
            Task::LaterUnloadTex(r) => self.unload_tex_later(r),
        }
    }

    fn replace_back(&mut self, new_back: Box<dyn Backend>) {
        let _ = mem::replace(&mut self.back, new_back);
    }

    fn load_tex(&mut self, task: LoadTexTask) {
        todo!()
    }

    fn unload_tex_later(&mut self, r: Receiver<LoadResult<TexId>>) {
        let task_sender = self.task_tx.clone();
        self.rt.spawn(async move {
            let result = r.try_recv();
            if let Ok(Ok(id)) = result {
                let _ = task_sender.send(Task::UnloadTex(id));
                return;
            }

            if let Err(TryRecvError::Empty) = result {
                tokio::time::delay_for(Duration::from_secs(1)).await;
                let _ = task_sender.send(Task::LaterUnloadTex(r));
                return;
            }
        });
    }
}

pub struct LoadTexTask {
    path: PathBuf,
    result_sender: Sender<LoadResult<Tex>>,
}
