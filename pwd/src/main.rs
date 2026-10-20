use posix_lib::{
    ExitCode, PosixContext, exit, get_environment_variable, get_option, posix_init, print_line,
};

fn pwd_main(_ctx: PosixContext) {
    let mut working_directory;

    if !get_option('P') {
        working_directory = get_environment_variable("PWD");

        if working_directory.len() + 1 > 256 {
            working_directory = get_physical_directory();
        }
    } else {
        working_directory = get_physical_directory();
    }

    print_line(&working_directory);

    exit(ExitCode::Success);
}

fn get_physical_directory() -> String {
    return "tbd get physical directory".to_string();
}

posix_init!(pwd_main);
