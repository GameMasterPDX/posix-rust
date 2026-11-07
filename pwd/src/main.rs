use posix_lib::{
    ExitCode, PosixContext,
    functions::{
        exit, get_current_working_directory, get_environment_variable, get_option, print_line,
    },
    posix_init,
    variables::PATH_MAX,
};

fn pwd_main(_ctx: PosixContext) {
    let mut working_directory;

    if !get_option('P') {
        working_directory = get_environment_variable("PWD");

        if working_directory.len() + 1 > PATH_MAX {
            working_directory = get_current_working_directory();
        }
    } else {
        working_directory = get_current_working_directory();
    }

    print_line(&working_directory);

    exit(ExitCode::Success);
}

posix_init!(pwd_main);
