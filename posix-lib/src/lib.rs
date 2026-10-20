use std::env;

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
///     posix_lib::exit(posix_lib::ExitCode::Success);
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

/// Exits the program with the `ExitCode` given
///
/// Exits the program with the `ExitCode` given, this will require manual cleanup of program
/// specific functionality as it uses `std::process::exit()` and as such does not clean up on its
/// own.
///
/// # Examples
///
/// ```rust
/// use posix_lib::exit;
///
/// exit(posix_lib::ExitCode::Success);
/// ```
pub fn exit(code: ExitCode) -> ! {
    let code_int = code as i32;

    std::process::exit(code_int);
}

/// Prints a line to standard output, followed by a new line.
///
/// See also [`println!`].
///
/// # Examples
///
/// ```rust
/// use posix_lib::print_line;
///
/// print_line("line");
/// ```
pub fn print_line(text: &str) {
    println!("{}", text);
}

/// Takes in the name of an environment variable and returns it's value, returns an empty string if
/// the variable is undefined.
///
/// # Examples
///
/// ```rust
/// use posix_lib::get_environment_variable;
///
/// println!("{}", get_environment_variable("PWD"));
/// ```
pub fn get_environment_variable(env_var: &str) -> String {
    match std::env::var(env_var) {
        Ok(val) => val,
        Err(_) => {
            print_line("error");
            String::new()
        }
    }
}

/// Takes in a character and returns a bool for if it is one of the flags given to the program.
///
/// Takes in only a single character, and doesn't see anything after "--" as per the POSIX
/// specification.
///
/// # Examples
///
/// ```rust
/// use posix_lib::get_option;
///
/// if get_option('A') && get_option('B') {
///     println!("Wow! Both A and B are flags!");
/// }
/// ```
pub fn get_option(option: char) -> bool {
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--" {
            break;
        }

        if arg.starts_with("-") && arg != "-" {
            if arg.contains(option) {
                return true;
            }
        }
    }

    return false;
}
