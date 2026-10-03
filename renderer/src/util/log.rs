use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging. The backend is provided by the host process.
/// Called once, safe to call multiple times.
pub fn init() {
    INIT.call_once(|| {
        // No-op unless a host provides a logger.
        // Log macros work regardless; they simply drop if no backend is set.
    });
}

/// Returns true if a logger is installed by the host.
pub fn is_enabled() -> bool {
    log::log_enabled!(log::Level::Info)
}

#[macro_export]
macro_rules! arashi_log {
    ($level:expr, $($arg:tt)*) => {
        log::log!($level, "[arashi] {}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! arashi_debug {
    ($($arg:tt)*) => { $crate::arashi_log!(log::Level::Debug, $($arg)*) };
}

#[macro_export]
macro_rules! arashi_info {
    ($($arg:tt)*) => { $crate::arashi_log!(log::Level::Info, $($arg)*) };
}

#[macro_export]
macro_rules! arashi_warn {
    ($($arg:tt)*) => { $crate::arashi_log!(log::Level::Warn, $($arg)*) };
}

#[macro_export]
macro_rules! arashi_error {
    ($($arg:tt)*) => { $crate::arashi_log!(log::Level::Error, $($arg)*) };
  }
