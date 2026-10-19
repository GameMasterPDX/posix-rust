use posix_lib::{ExitCode, PosixContext, exit};

fn true_main(ctx: PosixContext) {
    exit(ExitCode::Success);
}

posix_lib::posix_init!(true_main);
