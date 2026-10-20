pub const _POSIX_PATH_MAX: i32 = 256;
pub const _XOPEN_PATH_MAX: i32 = 1024;
#[cfg(feature = "xsi")]
pub const PATH_MAX: i32 = _XOPEN_PATH_MAX;
#[cfg(not(feature = "xsi"))]
pub const PATH_MAX: i32 = _POSIX_PATH_MAX;
