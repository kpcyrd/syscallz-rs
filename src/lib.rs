extern crate seccomp_sys;
extern crate libc;
#[macro_use] extern crate log;

use seccomp_sys::*;

mod error;
pub use error::{Error, Result};

mod syscalls;
pub use syscalls::Syscall;


#[derive(Debug)]
pub enum Action {
    Kill,
    Trap,
    Errno(u16),
    Trace(u16),
    Allow,
}

impl Into<u32> for Action {
    fn into(self) -> u32 {
        use self::Action::*;
        match self {
            Kill => SCMP_ACT_KILL,
            Trap => SCMP_ACT_TRAP,
            Errno(e) => SCMP_ACT_ERRNO(e.into()),
            Trace(t) => SCMP_ACT_TRACE(t.into()),
            Allow => SCMP_ACT_ALLOW,
        }
    }
}

pub struct Context {
    ctx: *mut scmp_filter_ctx,
}

impl Context {
    pub fn init() -> Result<Context> {
        Context::init_with_action(Action::Kill)
    }

    pub fn init_with_action(default_action: Action) -> Result<Context> {
        let ctx = unsafe { seccomp_init(default_action.into()) };

        if ctx.is_null() {
            return Err(Error::from("seccomp_init returned null".to_string()));
        }

        Ok(Context {
            ctx,
        })
    }

    #[inline]
    pub fn allow_syscall(&mut self, syscall: Syscall) -> Result<()> {
        self.set_action_for_syscall(Action::Allow, syscall)
    }

    #[inline]
    pub fn set_action_for_syscall(&mut self, action: Action, syscall: Syscall) -> Result<()> {
        debug!("seccomp: setting action={:?} syscall={:?}", action, syscall);
        let ret = unsafe { seccomp_rule_add(self.ctx, action.into(), syscall.into_i32(), 0) };

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
    use libc;
    use super::{Context, Action};
    use super::syscalls::Syscall;

    #[test]
    fn it_works() {
        let mut ctx = Context::init_with_action(Action::Errno(69)).unwrap();
        ctx.allow_syscall(Syscall::futex).unwrap();
        ctx.load().unwrap();
        assert_eq!(unsafe { libc::getpid() }, -69);
    }
}
