use crate::back::Backend;
use crate::task::Task;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct Context {
    pub back: Box<dyn Backend>,
    pub task_tx: Sender<Box<dyn Task>>,
    pub rt: Arc<Runtime>,
    pub current_tasks: Vec<JoinHandle<()>>,
}

pub struct Tasker {
    task_rx: Receiver<Box<dyn Task>>,
    ctx: Context,
}

impl Tasker {
    pub fn new(
        back: Box<dyn Backend>,
        task_tx: Sender<Box<dyn Task>>,
        task_rx: Receiver<Box<dyn Task>>,
    ) -> Self {
        let ctx = Context {
            back,
            task_tx,
            rt: Arc::new(Runtime::new().expect("Can't run tokio Runtime")),
            current_tasks: Vec::new(),
        };
        Self { task_rx, ctx }
    }

    pub fn start_next_task(&mut self) -> bool {
        // todo: update running tasks

        match self.task_rx.recv() {
            Ok(mut task) => {
                let running_task = task.start(&mut self.ctx);
                self.ctx.current_tasks.push(running_task);
                true
            }
            Err(_) => false,
        }
    }
}
