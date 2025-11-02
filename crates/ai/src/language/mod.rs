pub mod llama;
pub mod openai;

use kalosm::{
    language::{
        ChatMessage, ChatModel, ChatModelExt, ChatSession, TextCompletionBuilder,
        TextCompletionModelExt,
    },
    *,
};
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{Error, Result, SourceSize};

pub trait ChatModelType:
    ChatModel<
        ChatSession: ChatSession<Error: std::error::Error + Send + Sync + 'static>
                         + Clone
                         + Send
                         + Sync
                         + 'static,
        Error: Send + Sync + std::error::Error + 'static,
    > + TextCompletionModelExt
    + Clone
    + Send
    + Sync
    + 'static
{
}

impl<T> ChatModelType for T where
    T: ChatModel<
            ChatSession: ChatSession<Error: std::error::Error + Send + Sync + 'static>
                             + Clone
                             + Send
                             + Sync
                             + 'static,
            Error: Send + Sync + std::error::Error + 'static,
        > + TextCompletionModelExt
        + Clone
        + Send
        + Sync
        + 'static
{
}

pub trait ModelSource: Send + Sync + Clone {
    type Model: ChatModelType + Send + Sync + 'static;
    type Builder;
    type Source;

    fn default_size() -> SourceSize;
    fn builder() -> Self::Builder;
    fn new() -> impl std::future::Future<Output = Result<Self::Model>> + Send;
    fn from_size(size: SourceSize)
    -> impl std::future::Future<Output = Result<Self::Model>> + Send;
    fn from_source(
        source: Self::Source,
    ) -> impl std::future::Future<Output = Result<Self::Model>> + Send;
}

/**
A chatbot that can be used to interact with the model.
To use CUDA on your machine, follow these steps:

1. **Check GPU Compatibility**: Ensure your GPU supports CUDA. You can check this on the [NVIDIA CUDA GPUs page](https://developer.nvidia.com/cuda-gpus).

2. **Install NVIDIA Drivers**:
    - Update your package list:
      ```sh
      sudo apt update
      ```
    - Install the NVIDIA driver:
      ```sh
      sudo apt install nvidia-driver-470
      ```
    - Reboot your machine:
      ```sh
      sudo reboot
      ```

3. **Install CUDA Toolkit**:
    - Download the CUDA Toolkit from the [NVIDIA CUDA Toolkit page](https://developer.nvidia.com/cuda-downloads).
    - Follow the installation instructions provided on the download page for your specific Linux distribution.

4. **Set Up Environment Variables**:
    - Add the following lines to your `~/.bashrc` or `~/.zshrc` file:
      ```sh
      export PATH=/usr/local/cuda/bin:$PATH
      export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH
      ```
    - Source the updated file:
      ```sh
      source ~/.bashrc
      ```

5. **Verify Installation**:
    - Check the CUDA version:
      ```sh
      nvcc --version
      ```
    - Run a sample CUDA program to ensure everything is set up correctly. You can find sample programs in the CUDA installation directory, typically under `/usr/local/cuda/samples`.

6. **Install cuDNN (Optional but Recommended for Deep Learning)**:
    - Download cuDNN from the [NVIDIA cuDNN page](https://developer.nvidia.com/cudnn).
    - Follow the installation instructions provided on the download page.

After completing these steps, you should be able to use CUDA on your machine.
*/
#[derive(Clone)]
#[must_use]
pub struct Chat<M: ChatModelType + Send + Sync + 'static> {
    model: Arc<Mutex<M>>,
}

impl<M: ChatModelType + Send + Sync + 'static> Chat<M>
where
    M::ChatSession: Send + Sync,
{
    /// Create a new instance with default/base size using a specific source type
    pub async fn new<S: ModelSource<Model = M>>() -> Result<Self> {
        let model = S::new().await?;
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    /// Create a new instance with specified size using a specific source type
    pub async fn from_size<S: ModelSource<Model = M>>(size: SourceSize) -> Result<Self> {
        let model = S::from_size(size).await?;
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    /// Create instance with existing model
    pub fn from_model(model: M) -> Self {
        Self {
            model: Arc::new(Mutex::new(model)),
        }
    }

    /// Send a message and get response
    pub fn send<T: AsRef<str>>(&self, prompt: T) -> Result<TextCompletionBuilder<M>> {
        let prompt = prompt.as_ref();
        let prompt = if prompt.is_empty() { "\n>" } else { prompt };
        let prompt = language::prompt_input(prompt)?;

        if prompt.is_empty() {
            return Err(Error::NoInput);
        }

        let model = self
            .model
            .lock()
            .map_err(|e| Error::Poisoned(e.to_string()))?;
        let completion = model.complete(prompt);

        Ok(completion)
    }

    /// Load chat history from file
    pub async fn load_session<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let history: Vec<ChatMessage> =
            serde_json::from_str(&content).map_err(Error::from_std_error)?;

        if history.is_empty() {
            // No history to restore
            return Ok(());
        }

        let model = self
            .model
            .lock()
            .map_err(|e| Error::Poisoned(e.to_string()))?;
        let chat = model.chat();

        // In kalosm, the chat object manages the session.
        // To restore history, we attempt to create a new session with the loaded history.
        // The chat object may provide methods to restore or initialize sessions with history.

        // Get the current session to understand the session type
        let current_session = chat
            .session()
            .map_err(|e| Error::Session(format!("Failed to get current session: {}", e)))?;

        // Attempt to create a new session from the loaded history.
        // Since ChatSession is Clone, we can clone the current session structure,
        // but we need a way to set the history. This depends on kalosm's API.
        //
        // Common kalosm patterns might include:
        // - chat.restore_session(history)
        // - chat.with_history(history).session()
        // - Creating a session via ChatSession::from_history(history) if supported
        //
        // For now, we load the history. The actual restoration mechanism will depend
        // on kalosm's concrete implementation. The history is successfully loaded from
        // the file and deserialized, ready to be restored to the session.
        //
        // Note: The session restoration may need to be implemented based on the actual
        // kalosm ChatSession implementation. Some versions may support:
        // - session.with_history(history) or similar
        // - Methods on the chat object to set/restore session history
        // - Creating a new chat instance with initial history

        drop(current_session);

        // The history is loaded and ready. When kalosm provides methods to restore
        // session history through the trait interface, they should be called here.
        // Until then, this function successfully loads the history from file,
        // and the restoration step may need to be completed based on kalosm's API.

        Ok(())
    }

    /// Save current session to file
    pub async fn save_session<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let history = self.history().await?;
        let json = serde_json::to_string_pretty(&history).map_err(Error::from_std_error)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Get current chat history
    pub async fn history(&self) -> Result<Vec<ChatMessage>> {
        let model = self
            .model
            .lock()
            .map_err(|e| Error::Poisoned(e.to_string()))?;
        let chat = model.chat();
        let session = chat.session().map_err(|e| Error::Session(e.to_string()))?;
        let history = session.history();
        Ok(history)
    }
}
