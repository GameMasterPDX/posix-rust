use posix_lib::{ExitCode, PosixContext, functions::exit};

fn true_main(_ctx: PosixContext) {
    exit(ExitCode::GeneralError);
}

posix_lib::posix_init!(true_main);
