extern crate seccomp_sys;
extern crate libc;
#[macro_use] extern crate log;

use seccomp_sys::*;

mod error;
pub use error::{Error, Result};

mod syscalls;
pub use syscalls::Syscall;


pub struct Context {
    ctx: *mut scmp_filter_ctx,
}

impl Context {
    pub fn init() -> Result<Context> {
        let ctx = unsafe { seccomp_init(SCMP_ACT_KILL) };

        if ctx.is_null() {
            return Err(Error::from("seccomp_init returned null".to_string()));
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
            Err(Error::from("seccomp_rule_add returned error".to_string()))
        } else {
            Ok(())
        }
    }

    pub fn load(&self) -> Result<()> {
        let ret = unsafe { seccomp_load(self.ctx) };

        if ret != 0 {
            Err(Error::from("seccomp_load returned error".to_string()))
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
