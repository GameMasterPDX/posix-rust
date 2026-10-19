pub struct PosixContext {
    pub app_name: String,
}

impl PosixContext {
    pub fn init(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }
}

#[macro_export]
macro_rules! posix_init {
    ($real_main:ident) => {
        fn main() {
            let ctx = $crate::PosixContext::init(env!("CARGO_PKG_NAME"));

            $real_main(ctx);
        }
    };
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
}

pub fn exit(code: ExitCode) -> ! {
    let code_int = code as i32;

    std::process::exit(code_int);
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
