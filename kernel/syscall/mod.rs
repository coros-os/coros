///! Syscall handlers

pub use self::call::*;
pub use self::error::*;
pub use self::fs::*;
pub use self::process::*;
pub use self::validate::*;

/// System call numbers
mod call;

/// System error codes
mod error;

/// Filesystem syscalls
mod fs;

/// Process syscalls
mod process;

/// Validate input
mod validate;

#[no_mangle]
pub extern fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, stack: usize) -> usize {
    #[inline(always)]
    fn inner(a: usize, b: usize, c: usize, d: usize, e: usize, _f: usize, stack: usize) -> Result<usize> {
        match Call::from(a) {
            Ok(call) => match call {
                Call::Exit => exit(b),
                Call::Read => read(b, validate_slice_mut(c as *mut u8, d)?),
                Call::Write => write(b, validate_slice(c as *const u8, d)?),
                Call::Open => open(validate_slice(b as *const u8, c)?, d),
                Call::Close => close(b),
                Call::WaitPid => waitpid(b, c, d),
                Call::Exec => exec(validate_slice(b as *const u8, c)?, validate_slice(d as *const [usize; 2], e)?),
                Call::GetPid => getpid(),
                Call::Dup => dup(b),
                Call::Brk => brk(b),
                Call::Iopl => iopl(b),
                Call::Clone => clone(b, stack),
                Call::SchedYield => sched_yield()
            },
            Err(err) => {
                println!("Unknown syscall {}", a);
                Err(err)
            }
        }
    }

    match inner(a, b, c, d, e, f, stack) {
        Ok(value) => value,
        Err(value) => (-(value as isize)) as usize
    }
}
