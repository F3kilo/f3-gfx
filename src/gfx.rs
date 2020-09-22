use crate::back::Backend;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::{mem, thread};
use tokio::runtime::Runtime;

pub fn run(back: Box<dyn Backend>) -> Link {
    let (task_tx, task_rx) = mpsc::channel();
    let link = Link::new(task_tx);

    thread::spawn(move || {
        let mut tasker = Tasker::new(back, task_rx);
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
}

enum Task {
    ReplaceBack(Box<dyn Backend>),
}

struct Tasker {
    back: Box<dyn Backend>,
    task_rx: Receiver<Task>,
    rt: Arc<Runtime>,
}

impl Tasker {
    fn new(back: Box<dyn Backend>, task_rx: Receiver<Task>) -> Self {
        Self {
            back,
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
        }
    }

    fn replace_back(&mut self, new_back: Box<dyn Backend>) {
        let _ = mem::replace(&mut self.back, new_back);
    }
}
