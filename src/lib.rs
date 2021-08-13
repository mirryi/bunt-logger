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

pub use bunt::{
    self,
    termcolor::{ColorChoice, StandardStream, WriteColor},
};
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

/// Like [`log::error`], but bunt-compatible.
///
/// # Example
/// ```rust
/// use bunt_logger::error;
///
/// # fn main() {
/// let x = -1;
/// error!("{$red}Not a positive number:{/$} {[bold]}", x);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::error!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Error, writer => {
            let _ = $crate::bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

/// Like [`log::warn`], but bunt-compatible.
///
/// # Example
/// ```rust
/// use bunt_logger::warn;
///
/// # fn main() {
/// warn!("He likes {$yellow}lemons{/$}, like {$blue+italic}a lot{/$}.");
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::warn!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Warn, writer => {
            let _ = $crate::bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

/// Like [`log::info`], but bunt-compatible.
///
/// # Example
/// ```rust
/// use bunt_logger::info;
///
/// # fn main() {
/// let v = vec![1, 2, 3];
/// info!("Here is some data: {[green]:?}.", v);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::info!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Info, writer => {
            let _ = $crate::bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

/// Like [`log::debug`], but bunt-compatible.
///
/// # Example
/// ```rust
/// use bunt_logger::debug;
///
/// # fn main() {
/// let v = vec![1, 2, 3];
/// debug!("{$bold}Length: {[cyan]}{/$}.", v.len());
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::debug!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Debug, writer => {
            let _ = $crate::bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

/// Like [`log::trace`], but bunt-compatible.
///
/// # Example
/// ```rust
/// use bunt_logger::trace;
///
/// # fn main() {
/// let v = vec![1, 2, 3];
/// trace!("{$italic}Watch the mouse!{/$}.");
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    ($format_str:literal $(, $arg:expr)* $(,)?) => {
        $crate::trace!([$format_str] $(, $arg )*)
    };
    ([$($format_str:literal),+ $(,)?] $(, $arg:expr)* $(,)?) => {
        $crate::try_log!(Trace, writer => {
            let _ = $crate::bunt::writeln!(writer, [$($format_str)+] $(, $arg )*);
        })
    }
}

static LOGPREFS: Lazy<Mutex<LogPrefs>> = Lazy::new(|| {
    let prefs = LogPrefs::new();
    Mutex::new(prefs)
});

/// Returns a reference to the global preferences object, used for modifying preferences.
///
/// # Example
/// ```rust
/// use bunt_logger::{ColorChoice, Level};
///
/// fn main() {
///     bunt_logger::with()
///         .level(Level::Debug)
///         .stdout(ColorChoice::Never);
/// }
/// ```
#[inline]
pub fn with() -> MutexGuard<'static, LogPrefs> {
    LOGPREFS.lock().unwrap()
}

/// Preferences that dictate logging.
pub struct LogPrefs {
    quiet: bool,
    filter: LevelFilter,

    writer: Box<dyn WriteColor + Send>,
}

impl LogPrefs {
    #[inline]
    fn new() -> Self {
        Self {
            quiet: false,
            filter: LevelFilter::Info,
            writer: Box::new(StandardStream::stdout(ColorChoice::Auto)),
        }
    }

    /// Sets whether all output should be silenced, regardless of log level.
    ///
    /// # Example
    /// ```rust
    /// # fn main() {
    /// bunt_logger::with().quiet(true);
    /// # }
    /// ```
    #[inline]
    pub fn quiet(&mut self, quiet: bool) -> &mut Self {
        self.quiet = quiet;
        self
    }

    /// Sets the log level.
    ///
    /// # Example
    /// ```rust
    /// use bunt_logger::Level;
    ///
    /// # fn main() {
    /// bunt_logger::with().level(Level::Debug);
    /// # }
    /// ```
    #[inline]
    pub fn level(&mut self, level: Level) -> &mut Self {
        self.filter = level.to_level_filter();
        self
    }

    /// Sets the logging target.
    ///
    /// By default, `StandardStream::stdout(ColorChoice::Auto)` is used.
    ///
    /// # Example
    /// ```rust
    /// use bunt_logger::{ColorChoice, StandardStream};
    ///
    /// # fn main() {
    /// let stderr_writer = StandardStream::stderr(ColorChoice::Never);
    /// bunt_logger::with()
    ///     .writer(Box::new(stderr_writer));
    /// # }
    /// ```
    #[inline]
    pub fn writer(&mut self, writer: Box<dyn WriteColor + Send + Sync>) -> &mut Self {
        self.writer = writer;
        self
    }

    /// Sets the logging target to stdout with the given [`ColorChoice`].
    ///
    /// # Example
    /// ```rust
    /// use bunt_logger::ColorChoice;
    ///
    /// # fn main() {
    /// bunt_logger::with()
    ///     .stdout(ColorChoice::Always);
    /// # }
    /// ```
    #[inline]
    pub fn stdout(&mut self, color: ColorChoice) -> &mut Self {
        self.writer(Box::new(StandardStream::stdout(color)))
    }

    /// Sets the logging target to stderr with the given [`ColorChoice`].
    ///
    /// # Example
    /// ```rust
    /// use bunt_logger::ColorChoice;
    ///
    /// # fn main() {
    /// bunt_logger::with()
    ///     .stderr(ColorChoice::Always);
    /// # }
    /// ```
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
