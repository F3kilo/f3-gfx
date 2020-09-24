pub mod load_tex;

use crate::gfx::Context;
use log::error;
use tokio::task::JoinHandle;

pub trait Task: Send {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()>;
}

fn task_started_twice_error(task_name: &'static str) -> JoinHandle<()> {
    error!("Task was started twice: {}", task_name);
    panic!("Task was started twice: {}", task_name)
}
