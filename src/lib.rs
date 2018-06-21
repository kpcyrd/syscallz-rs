extern crate seccomp_sys;
extern crate libc;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use seccomp_sys::*;

mod syscalls;
pub use syscalls::Syscall;

mod errors {
    error_chain! {
    }
}
pub use errors::{Error, ErrorKind, Result};


pub struct Context {
    ctx: *mut scmp_filter_ctx,
}

impl Context {
    pub fn init() -> Result<Context> {
        let ctx = unsafe { seccomp_init(SCMP_ACT_KILL) };

        if ctx.is_null() {
            bail!("seccomp_init returned null");
        }

        Ok(Context {
            ctx,
        })
    }

    #[inline]
    pub fn allow_syscall(&mut self, syscall: Syscall) -> Result<()> {
        debug!("seccomp: allowing syscall={:?}", syscall);
        let ret = unsafe { seccomp_rule_add(self.ctx, SCMP_ACT_ALLOW, syscall.into_i32(), 0) };

        if ret != 0 {
            bail!("seccomp_rule_add returned error");
        } else {
            Ok(())
        }
    }

    pub fn load(&self) -> Result<()> {
        let ret = unsafe { seccomp_load(self.ctx) };

        if ret != 0 {
            bail!("seccomp_load returned error");
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            seccomp_release(self.ctx)
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
