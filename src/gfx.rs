use crate::back::Backend;
use crate::running::RunningTasks;
use crate::task::Task;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Context {
    pub back: Box<dyn Backend>,
    pub task_tx: Sender<Box<dyn Task>>,
    pub rt: Arc<Runtime>,
    pub running: RunningTasks,
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
            running: RunningTasks::default(),
        };
        Self { task_rx, ctx }
    }

    pub fn start_next_task(&mut self) -> bool {
        match self.task_rx.recv() {
            Ok(mut task) => {
                let running_task = task.start(&mut self.ctx);
                self.ctx.running.add(running_task);
                true
            }
            Err(_) => false,
        }
    }

    fn refresh_runing_tasks(&mut self) {
        const MAX_RUNNING_TASKS_COUNT: usize = 128;
        if self.ctx.running.len() > MAX_RUNNING_TASKS_COUNT {
            self.ctx.running.refresh();
        }
    }
}
