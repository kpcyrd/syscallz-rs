extern crate libc;
extern crate seccomp_sys;
#[macro_use]
extern crate log;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use seccomp_sys::*;

mod rule;
pub use rule::{Cmp, Comparator};

mod error;
pub use error::{Error, Result};

mod syscalls;
pub use syscalls::Syscall;

#[derive(Debug, Clone, Copy)]
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
    /// Initialize a context with Action::Kill.
    pub fn init() -> Result<Context> {
        Context::init_with_action(Action::Kill)
    }

    /// Initialize a context with default_action.
    ///
    /// Returns Err(Error::Msg) if seccomp_init returned NULL.
    pub fn init_with_action(default_action: Action) -> Result<Context> {
        let ctx = unsafe { seccomp_init(default_action.into()) };

        if ctx.is_null() {
            return Err(Error::from("seccomp_init returned null".to_string()));
        }

        Ok(Context { ctx })
    }

    #[inline]
    pub fn allow_syscall(&mut self, syscall: Syscall) -> Result<()> {
        self.set_action_for_syscall(Action::Allow, syscall)
    }

    /// Returns Err(Error::Os) if seccomp_rule_add failed.
    pub fn set_action_for_syscall(&mut self, action: Action, syscall: Syscall) -> Result<()> {
        debug!("seccomp: setting action={:?} syscall={:?}", action, syscall);
        let ret = unsafe { seccomp_rule_add(self.ctx, action.into(), syscall.into_i32(), 0) };

        if ret != 0 {
            Err(Error::from_errno(-ret))
        } else {
            Ok(())
        }
    }

    /// Returns Err(Error::Os) if seccomp_rule_add_array failed.
    ///
    /// Due to [seccomp/libseccomp #118](https://github.com/seccomp/libseccomp/issues/118),
    /// this method only allows you to specify one comparison per argument.
    pub fn set_rule_for_syscall(
        &mut self,
        action: Action,
        syscall: Syscall,
        comparators: &[Comparator],
    ) -> Result<()> {
        debug!(
            "seccomp: setting action={:?} syscall={:?} comparators={:?}",
            action, syscall, comparators
        );
        let ret = unsafe {
            seccomp_rule_add_array(
                self.ctx,
                action.into(),
                syscall.into_i32(),
                comparators.len() as u32,
                comparators.as_ptr() as *const scmp_arg_cmp,
            )
        };
        if ret != 0 {
            Err(Error::from_errno(-ret))
        } else {
            Ok(())
        }
    }

    /// Loads the seccomp filter into the kernel.
    ///
    /// Returns Err(Error::Os) if seccomp_load failed.
    pub fn load(&self) -> Result<()> {
        debug!("seccomp: loading policy");
        let ret = unsafe { seccomp_load(self.ctx) };

        if ret != 0 {
            Err(Error::from_errno(-ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { seccomp_release(self.ctx) };
    }
}

#[cfg(test)]
mod tests {
    use super::syscalls::Syscall;
    use super::{Action, Context};
    use libc;

    // this test isn't fully stable yet
    #[test]
    #[ignore]
    fn it_works() {
        let mut ctx = Context::init_with_action(Action::Errno(69)).unwrap();
        ctx.allow_syscall(Syscall::futex).unwrap();
        ctx.load().unwrap();
        assert_eq!(unsafe { libc::getpid() }, -69);
    }

    #[test]
    fn from_name() {
        use crate::syscalls::Syscall;

        let cases = vec![
            ("open", Some(Syscall::open)),
            ("setgid", Some(Syscall::setgid)),
            ("nothing", None),
            ("", None),
        ];

        for (name, rhs) in cases {
            let lhs = Syscall::from_name(name);
            assert_eq!(lhs, rhs);
        }
    }

    #[test]
    fn test_rule() {
        use crate::rule::{Cmp, Comparator};
        use crate::Action;
        use std::fs::File;
        use std::io::Read;
        use std::os::unix::io::AsRawFd;

        let mut f = File::open("Cargo.toml").unwrap();

        let mut ctx = Context::init_with_action(Action::Allow).unwrap();
        ctx.set_rule_for_syscall(
            Action::Errno(1),
            Syscall::read,
            &[Comparator::new(0, Cmp::Eq, f.as_raw_fd() as u64, 0)],
        )
        .unwrap();
        ctx.load().unwrap();

        let mut buf: [u8; 1024] = [0; 1024];
        let res = f.read(&mut buf);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(1));
    }
}
