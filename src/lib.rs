use std::fmt;
use thiserror::Error;

/// Gfx frontend task
#[derive(Debug)]
pub enum GfxTask {
    Backend(BackendTask),
    Command(ServiceTask),
}

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    LoadResource,
    Present,
}

/// Gfx command task
#[derive(Debug)]
pub enum ServiceTask {
    ChangeBackend(Box<dyn GfxBackend>),
}

/// Gfx frontend
pub trait Gfx {
    /// Run enqueued tasks
    fn run_tasks(&mut self);
    fn is_working(&self) -> bool;
}

/// Gfx frontend builder
pub struct GfxBuilder<TaskReceiver: GfxTaskReceiver> {
    tasks: TaskReceiver,
    backend: Box<dyn GfxBackend>,
}

impl<TaskReceiver: GfxTaskReceiver> GfxBuilder<TaskReceiver> {
    /// Builds frontend with specified parameters
    pub fn build(self) -> impl Gfx {
        let gfx = WorkingGenericGfx::new(self.backend, self.tasks);
        GenericGfx::Working(gfx)
    }
}

/// Generic Gfx frontend
enum GenericGfx<TaskReceiver: GfxTaskReceiver> {
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
struct WorkingGenericGfx<TaskReceiver: GfxTaskReceiver> {
    backend: Box<dyn GfxBackend>,
    tasks: TaskReceiver,
}

impl<TaskReceiver: GfxTaskReceiver> WorkingGenericGfx<TaskReceiver> {
    fn new(backend: Box<dyn GfxBackend>, tasks: TaskReceiver) -> Self {
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

/// Trait source of gfx tasks
pub trait GfxTaskReceiver {
    /// Get new tasks if some present
    fn pop(&mut self) -> Result<Option<GfxTask>, ReceiveTaskError>;
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug + Send {
    fn run_task(&mut self, task: BackendTask);
}

/// Error with recieving tasks
#[derive(Debug, Error)]
pub enum ReceiveTaskError {
    #[error("all input channels to Gfx dropped")]
    LostAllInputChannels,
}

/// Error with running tasks
#[derive(Debug, Error)]
enum RunTasksError {
    #[error(transparent)]
    CantReceiveTasks(#[from] ReceiveTaskError),
}
