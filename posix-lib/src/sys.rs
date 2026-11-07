pub const SYS_READ: i32 = 0;
pub const SYS_WRITE: i32 = 1;
pub const SYS_EXIT: i32 = 60;

#[macro_export]
macro_rules! syscall {
  ($syscall:expr) => {
    unsafe {
      let res: i32;
      core::arch::asm!(
        "syscall",
        inout("rax") $syscall => res,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags),
      );
      res
    }
  };

  ($syscall:expr, $arg0:expr) => {
    unsafe {
      let res: i32;
      core::arch::asm!(
        "syscall",
        inout("rax") $syscall => res,
        in("rdi") $arg0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags),
      );
      res
    }
  };

  ($syscall:expr, $arg0:expr, $arg1:expr) => {
    unsafe {
      let res: i32;
      core::arch::asm!(
        "syscall",
        inout("rax") $syscall => res,
        in("rdi") $arg0,
        in("rsi") $arg1,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags),
      );
      res
    }
  };

  ($syscall:expr, $arg0:expr, $arg1:expr, $arg2:expr) => {
    unsafe {
      let res: i32;
      core::arch::asm!(
        "syscall",
        inout("rax") $syscall => res,
        in("rdi") $arg0,
        in("rsi") $arg1,
        in("rdx") $arg2,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags),
      );
      res
    }
  };
}
