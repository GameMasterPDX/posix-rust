use std::env;

use crate::ExitCode;
use crate::syscall;
use crate::sys::SYS_WRITE;
use crate::sys::SYS_EXIT;

/// Exits the program with the `ExitCode` given
///
/// Exits the program with the `ExitCode` given, this will require manual cleanup of program
/// specific functionality as it uses `std::process::exit()` and as such does not clean up on its
/// own.
///
/// # Examples
///
/// ```rust
/// use posix_lib::functions::exit;
///
/// exit(posix_lib::ExitCode::Success);
/// ```
pub fn exit(code: ExitCode) -> ! {
    let code_int = code as i32;

    let _res = syscall!(SYS_EXIT, code_int);

    loop {}
}

/// Prints the input text to STDOUT.
///
/// See also [`print!`].
///
/// # Examples
///
/// ```rust
/// use posix_lib::functions::print;
///
/// print("text");
/// ```
pub fn print(text: &str) {
    syscall!(SYS_WRITE, 1, text.as_ptr(), text.len());
}


/// Prints a line to STDOUT, followed by a new line.
///
/// See also [`println!`].
///
/// # Examples
///
/// ```rust
/// use posix_lib::functions::print_line;
///
/// print_line("line");
/// ```
pub fn print_line(text: &str) {
    print(text);
    print("\n");
}

/// Takes in the name of an environment variable and returns it's value, returns an empty string if
/// the variable is undefined.
///
/// # Examples
///
/// ```rust
/// use posix_lib::functions::get_environment_variable;
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
/// use posix_lib::functions::get_option;
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

/// Returns the current physical directory as a string.
///
/// # Examples
///
/// ```rust
/// use posix_lib::functions::get_current_working_directory;
///
/// let cwd = get_current_working_directory();
/// ```
pub fn get_current_working_directory() -> String {
    return env::current_dir().unwrap().display().to_string();
}
