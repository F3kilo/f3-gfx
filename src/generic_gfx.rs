use crate::back::GfxBackend;
use crate::task_recv::{ReceiveTask, ReceiveTaskError};
use crate::{Gfx, GfxError, GfxTask, ServiceTask};
use thiserror::Error;

/// Generic Gfx frontend
pub(crate) enum GenericGfx<TaskReceiver: ReceiveTask> {
    Working(WorkingGenericGfx<TaskReceiver>),
    Finished,
}

impl<TaskReceiver: ReceiveTask> GenericGfx<TaskReceiver> {
    fn process_run_task_result(
        &mut self,
        result: Result<(), RunTasksError>,
    ) -> Result<(), GfxError> {
        if let Err(e) = result {
            log::info!("Finishing gfx work: {}.", e);
            *self = Self::Finished;

            return match e {
                RunTasksError::CantReceiveTasks(_) => Ok(()),
                RunTasksError::GfxError(e) => Err(e),
            };
        }
        Ok(())
    }
}

impl<TaskQueue: ReceiveTask> Gfx for GenericGfx<TaskQueue> {
    fn run_tasks(&mut self) -> Result<(), GfxError> {
        if let Self::Working(working) = self {
            let run_tasks_result = working.run_tasks();
            return self.process_run_task_result(run_tasks_result);
        }
        Ok(())
    }

    fn is_working(&self) -> bool {
        matches!(self, Self::Working(_))
    }

    fn update(&mut self) -> Result<bool, GfxError> {
        if let Self::Working(working) = self {
            return working.update();
        }
        Ok(false)
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
            if let Err(e) = match task {
                GfxTask::Backend(t) => self.backend.run_task(t),
                GfxTask::Command(t) => self.run_service_task(t),
            } {
                return Err(e.into());
            }
        }
        Ok(())
    }

    fn update(&mut self) -> Result<bool, GfxError> {
        self.backend.update()
    }

    fn run_service_task(&mut self, task: ServiceTask) -> Result<(), GfxError> {
        match task {
            ServiceTask::ChangeBackend(new_back) => self.change_backend(new_back),
        }
    }

    fn change_backend(&mut self, _new_back: Box<dyn GfxBackend>) -> Result<(), GfxError> {
        todo!("copy all data from old back to new and swap self.backend with new_back")
    }
}

/// Error with running tasks
#[derive(Debug, Error)]
enum RunTasksError {
    #[error(transparent)]
    CantReceiveTasks(#[from] ReceiveTaskError),
    #[error(transparent)]
    GfxError(#[from] GfxError),
}
