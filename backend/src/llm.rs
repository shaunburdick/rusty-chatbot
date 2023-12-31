use std::path::PathBuf;

use llm::{models::Llama, LoadError, ModelParameters};

/// LLM Wrapper
pub struct Llm {
    model: Llama,
}

impl Llm {
    /// Create a new instance of the LLM
    ///
    /// Arguments:
    /// - model_path: A path to the modal to load
    /// - model_config: Configuration for the Llama model
    pub fn new(model_path: &str, model_config: ModelParameters) -> Result<Self, LoadError> {
        let model = llm::load::<Llama>(
            &PathBuf::from(model_path),
            model_config,
            llm::load_progress_callback_stdout,
        )?;

        Ok(Self { model })
    }
}
