use crate::GfxTask;
use thiserror::Error;

/// Trait source of gfx tasks
pub trait ReceiveTask: Send {
    /// Get new tasks if some present
    fn pop(&mut self) -> Result<Option<GfxTask>, ReceiveTaskError>;
}

/// Error with recieving tasks
#[derive(Debug, Error)]
pub enum ReceiveTaskError {
    #[error("all input channels to Gfx dropped")]
    LostAllInputChannels,
}
