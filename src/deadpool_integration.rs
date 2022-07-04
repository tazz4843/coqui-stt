pub use deadpool::managed::reexports::*;
pub use deadpool_sync::reexports::*;

use deadpool::managed::{Manager, RecycleResult};
use deadpool_sync::SyncWrapper;

/// A `deadpool` wrapper for Models.
pub struct DeadpoolModelWrapper {
    model_path: String,
    scorer_path: Option<String>,
    runtime: Runtime,
}

impl DeadpoolModelWrapper {
    /// Create a new DeadpoolModelWrapper.
    ///
    /// # Arguments
    /// * `model_path` - Path to the model.
    /// * `scorer_path` - Path to the scorer. Optional.
    pub fn new(
        model_path: impl Into<String>,
        scorer_path: Option<impl Into<String>>,
        runtime: Runtime,
    ) -> Self {
        Self {
            model_path: model_path.into(),
            scorer_path: scorer_path.map(Into::into),
            runtime,
        }
    }
}

pub enum DeadpoolModelWrapperError {
    Stt(crate::Error),
    Deadpool(InteractError),
}

impl From<crate::Error> for DeadpoolModelWrapperError {
    fn from(err: crate::Error) -> Self {
        Self::Stt(err)
    }
}

impl From<InteractError> for DeadpoolModelWrapperError {
    fn from(err: InteractError) -> Self {
        Self::Deadpool(err)
    }
}

#[async_trait::async_trait]
impl Manager for DeadpoolModelWrapper {
    type Type = SyncWrapper<crate::Model>;
    type Error = DeadpoolModelWrapperError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let mut m = SyncWrapper::new(self.runtime, || crate::Model::new(&self.model_path)).await?;
        if let Some(scorer_path) = &self.scorer_path {
            m.interact(|m| m.enable_external_scorer(scorer_path))
                .await??;
        }

        Ok(m)
    }

    async fn recycle(&self, _: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}
