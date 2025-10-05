pub mod errors;

use std::sync::OnceLock;

use errors::EMError;

pub type Result<T> = std::result::Result<T, EMError>;

static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn set_debug(value: bool) {
    DEBUG
        .set(value)
        .expect("Debug flag mode was already initialized.");
}

pub fn is_debug() -> bool {
    *DEBUG.get().unwrap_or(&false)
}

pub trait CallbackHandler<T> {
    fn starting(&self);
    fn update(&self, data: T);
    fn completed(&self);
}

pub mod system {
    pub fn num_cpus() -> usize {
        if super::is_debug() {
            1
        } else {
            num_cpus::get()
        }
    }
}
