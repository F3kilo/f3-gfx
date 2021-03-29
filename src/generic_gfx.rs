use crate::back::GfxBackend;
use crate::task_recv::{GfxTaskReceiver, ReceiveTaskError};
use crate::{Gfx, GfxTask, ServiceTask};
use thiserror::Error;

/// Generic Gfx frontend
pub(crate) enum GenericGfx<TaskReceiver: GfxTaskReceiver> {
    Working(WorkingGenericGfx<TaskReceiver>),
    Finished,
}

impl<TaskReceiver: GfxTaskReceiver> GenericGfx<TaskReceiver> {
    fn process_run_task_result(&mut self, result: Result<(), RunTasksError>) {
        if let Err(e) = result {
            log::info!("Finishing gfx work: {}.", e);
            *self = Self::Finished;
        }
    }
}

impl<TaskQueue: GfxTaskReceiver> Gfx for GenericGfx<TaskQueue> {
    fn run_tasks(&mut self) {
        if let Self::Working(working) = self {
            let run_tasks_result = working.run_tasks();
            self.process_run_task_result(run_tasks_result);
        }
    }

    fn is_working(&self) -> bool {
        matches!(self, Self::Working(_))
    }
}

/// Graphics frontend in work
pub(crate) struct WorkingGenericGfx<TaskReceiver: GfxTaskReceiver> {
    backend: Box<dyn GfxBackend>,
    tasks: TaskReceiver,
}

impl<TaskReceiver: GfxTaskReceiver> WorkingGenericGfx<TaskReceiver> {
    pub fn new(backend: Box<dyn GfxBackend>, tasks: TaskReceiver) -> Self {
        Self { backend, tasks }
    }

    fn run_tasks(&mut self) -> Result<(), RunTasksError> {
        while let Some(task) = self.tasks.pop()? {
            match task {
                GfxTask::Backend(t) => self.backend.run_task(t),
                GfxTask::Command(t) => self.run_service_task(t),
            }
        }
        Ok(())
    }

    fn run_service_task(&mut self, task: ServiceTask) {
        match task {
            ServiceTask::ChangeBackend(new_back) => self.change_backend(new_back),
        }
    }

    fn change_backend(&mut self, _new_back: Box<dyn GfxBackend>) {
        todo!("copy all data from old back to new and swap self.backend with new_back")
    }
}

/// Error with running tasks
#[derive(Debug, Error)]
enum RunTasksError {
    #[error(transparent)]
    CantReceiveTasks(#[from] ReceiveTaskError),
}
