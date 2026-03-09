use std::sync::LazyLock;

pub mod crash;
pub mod mouse;
pub mod process;

pub static NO_REQUESTS: LazyLock<bool> = LazyLock::new(|| std::env::var("NO_REQUESTS").is_ok());
