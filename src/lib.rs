//! A simple logger wrapper around [bunt](https://github.com/LukasKalbertodt/bunt).
//!
//! # Usage
//!
//! ```rust
//! use bunt_logger::{
//!     debug, error, info, trace, warn,
//!     ColorChoice,
//!     Level, // re-export of `log::Level`
//! };
//!
//! fn main() {
//!     bunt_logger::with()
//!         .level(Level::Trace)
//!         .stderr(ColorChoice::Always);
//!
//!     error!("{$red+bold}A red and bold error message!{/$}");
//!     warn!("{$yellow}A yellow warning message!{/$}");
//!     info!("{$green}A green info message!{/$}");
//!     debug!("{$cyan}A cyan debug message!{/$}");
//!     trace!("{$white+dimmed}A white and dimmed trace message!{/$}");
//! }
//! ```

use std::sync::{Mutex, MutexGuard};

use log::LevelFilter;
use once_cell::sync::Lazy;

pub use bunt::termcolor::{ColorChoice, StandardStream, WriteColor};
pub use log::Level;

#[doc(hidden)]
#[macro_export]
macro_rules! try_log {
    ($level:ident, $writer:ident => $b:block) => {{
        let mut prefs = $crate::with();
        if prefs.enabled($crate::Level::$level) {
            let mut $writer = prefs.get_writer();
            $b
        }
    }};
}

#[macro_export]
macro_rules! error {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::error!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Error, writer => {
            let _ = bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

#[macro_export]
macro_rules! warn {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::warn!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Warn, writer => {
            let _ = bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

#[macro_export]
macro_rules! info {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::info!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Info, writer => {
            let _ = bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

#[macro_export]
macro_rules! debug {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::debug!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Debug, writer => {
            let _ = bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

#[macro_export]
macro_rules! trace {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::trace!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Trace, writer => {
            let _ = bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

static LOGPREFS: Lazy<Mutex<LogPrefs>> = Lazy::new(|| {
    let prefs = LogPrefs::new();
    Mutex::new(prefs)
});

#[inline]
pub fn with() -> MutexGuard<'static, LogPrefs> {
    LOGPREFS.lock().unwrap()
}

#[inline]
pub fn new() -> LogPrefs {
    LogPrefs::new()
}

pub struct LogPrefs {
    quiet: bool,
    filter: LevelFilter,

    writer: Box<dyn WriteColor + Send>,
}

impl LogPrefs {
    #[inline]
    pub fn new() -> Self {
        Self {
            quiet: false,
            filter: LevelFilter::Info,
            writer: Box::new(StandardStream::stdout(ColorChoice::Auto)),
        }
    }

    #[inline]
    pub fn quiet(&mut self, quiet: bool) -> &mut Self {
        self.quiet = quiet;
        self
    }

    #[inline]
    pub fn level(&mut self, level: Level) -> &mut Self {
        self.filter = level.to_level_filter();
        self
    }

    #[inline]
    pub fn writer(&mut self, writer: Box<dyn WriteColor + Send + Sync>) -> &mut Self {
        self.writer = writer;
        self
    }

    #[inline]
    pub fn stdout(&mut self, color: ColorChoice) -> &mut Self {
        self.writer(Box::new(StandardStream::stdout(color)))
    }

    #[inline]
    pub fn stderr(&mut self, color: ColorChoice) -> &mut Self {
        self.writer(Box::new(StandardStream::stderr(color)))
    }

    #[doc(hidden)]
    #[inline]
    pub fn enabled(&self, level: Level) -> bool {
        !self.quiet && self.filter >= level
    }

    #[doc(hidden)]
    #[inline]
    pub fn get_writer<'a>(&'a mut self) -> &'a mut Box<dyn WriteColor + Send> {
        &mut self.writer
    }
}
