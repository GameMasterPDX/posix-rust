pub mod functions;
pub mod variables;
pub mod sys;

/// A global context designed to be used in POSIX programs.
///
/// A context designed to fulfill the functions needed for the programs defined under the POSIX
/// specification. Holds the `app_name` string which contains the name of the application to be used
/// elsewhere.
///
/// # Examples
///
/// ```rust
/// use posix_lib::PosixContext;
///
/// let ctx = PosixContext::init("name");
///
/// println!("{}", ctx.app_name);
/// ```
pub struct PosixContext {
    pub app_name: String,
}

impl PosixContext {
    /// Initializes the PosixContext this is run for. Takes in a `app_name` string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use posix_lib::PosixContext;
    ///
    /// let ctx = PosixContext::init("name");
    ///
    /// println!("{}", ctx.app_name);
    /// ```
    pub fn init(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }
}

/// Initializes the program and provides a context for it to use.
///
/// Takes in your program's main function and runs it with a generated context for usage. This macro
/// is mostly useful in CLI programs.
///
/// # Examples
///
/// ```rust,no_run
/// fn util_main(ctx: posix_lib::PosixContext) -> ! {
///     println!("Example");
///     posix_lib::functions::exit(posix_lib::ExitCode::Success);
/// }
///
/// posix_lib::posix_init!(util_main);
/// ```
#[macro_export]
macro_rules! posix_init {
    ($real_main:ident) => {
        fn main() {
            let binary_name = std::env::args()
                .next()
                .and_then(|s| s.rsplit(&['/', '\\'][..]).next().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown".to_string());

            let ctx = $crate::PosixContext::init(&binary_name);

            $real_main(ctx);
        }
    };
}

/// Represents the status code of the program.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExitCode {
    /// A general successful exit.
    Success = 0,

    /// A general unsuccessful exit.
    GeneralError = 1,
}
