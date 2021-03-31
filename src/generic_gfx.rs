use crate::back::GfxBackend;
use crate::task_recv::{ReceiveTask, ReceiveTaskError};
use crate::{Gfx, GfxTask, ServiceTask};
use thiserror::Error;

/// Generic Gfx frontend
pub(crate) enum GenericGfx<TaskReceiver: ReceiveTask> {
    Working(WorkingGenericGfx<TaskReceiver>),
    Finished,
}

impl<TaskReceiver: ReceiveTask> GenericGfx<TaskReceiver> {
    fn process_run_task_result(&mut self, result: Result<(), RunTasksError>) {
        if let Err(e) = result {
            log::info!("Finishing gfx work: {}.", e);
            *self = Self::Finished;
        }
    }
}

impl<TaskQueue: ReceiveTask> Gfx for GenericGfx<TaskQueue> {
    fn run_tasks(&mut self) {
        if let Self::Working(working) = self {
            let run_tasks_result = working.run_tasks();
            self.process_run_task_result(run_tasks_result);
        }
    }

    fn is_working(&self) -> bool {
        matches!(self, Self::Working(_))
    }

    fn update(&mut self) {
        if let Self::Working(working) = self {
            working.update();
        }
    }
}

/// Graphics frontend in work
pub(crate) struct WorkingGenericGfx<TaskReceiver: ReceiveTask> {
    backend: Box<dyn GfxBackend>,
    tasks: TaskReceiver,
}

impl<TaskReceiver: ReceiveTask> WorkingGenericGfx<TaskReceiver> {
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

    fn update(&mut self) {
        self.backend.poll_tasks();
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
