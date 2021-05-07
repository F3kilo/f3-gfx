pub mod back;
pub mod handler;
pub mod res;
pub mod scene;
mod task_channel;

use crate::task_channel::TasksChannel;
use back::{BackendTask, GfxBackend};
use crate::handler::GfxHandler;
use crate::back::GfxBackendUpdateError;
use slog::Logger;

/// Gfx frontend task.
#[derive(Debug)]
pub enum GfxTask {
    Backend(BackendTask),
    Service(ServiceTask),
}

/// Gfx command task.
#[derive(Debug)]
pub enum ServiceTask {
    ChangeBackend(Box<dyn GfxBackend>),
}

/// Gfx frontend.
pub struct Gfx {
    tasks_channel: TasksChannel,
    backend: Box<dyn GfxBackend>,
    logger: Logger,
}

fn default_logger() -> Logger {
    Logger::root(slog::Discard, slog::o!())
}

impl Gfx {
    /// Creates new graphics frontend.
    pub fn new(backend: Box<dyn GfxBackend>, logger: Option<Logger>) -> Self {
        let logger = logger.unwrap_or_else(default_logger);
        slog::trace!(logger, "Creatin new Gfx.");
        Self {
            backend,
            tasks_channel: TasksChannel::default(),
            logger,
        }
    }

    pub fn create_handler(&self) -> GfxHandler {
        GfxHandler::new(self.tasks_channel.sender())
    }

    /// Pushes enqueued tasks.
    fn push_tasks(&mut self) {
        slog::trace!(self.logger, "Pushing Gfx tasks.");
        while let Some(task) = self.tasks_channel.pop() {
            slog::trace!(self.logger, "Running gfx task: {:?}.", task);
            match task {
                GfxTask::Backend(t) => self.backend.run_task(t),
                GfxTask::Service(t) => self.run_service_task(t),
            }
        }
    }

    /// Update graphics. Some resources may be sent to consumers.
    pub fn update(&mut self) -> Result<(), GfxBackendUpdateError> {
        self.push_tasks();

        slog::trace!(self.logger, "Updating Gfx.");
        self.backend.update()?;
        Ok(())
    }

    fn run_service_task(&mut self, _task: ServiceTask) {
        todo!()
    }
}
