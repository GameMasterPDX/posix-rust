pub const _POSIX_PATH_MAX: usize = 256;
pub const _XOPEN_PATH_MAX: usize = 1024;
#[cfg(feature = "xsi")]
pub const PATH_MAX: usize = _XOPEN_PATH_MAX;
#[cfg(not(feature = "xsi"))]
pub const PATH_MAX: usize = _POSIX_PATH_MAX;
