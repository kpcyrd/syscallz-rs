use strum_macros::EnumString;
use libc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[allow(non_camel_case_types)]
pub enum Syscall {
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
_llseek = libc::SYS__llseek as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
_newselect = libc::SYS__newselect as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
_sysctl = libc::SYS__sysctl as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86_64"))))]
accept = libc::SYS_accept as isize,
accept4 = libc::SYS_accept4 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
access = libc::SYS_access as isize,
acct = libc::SYS_acct as isize,
add_key = libc::SYS_add_key as isize,
adjtimex = libc::SYS_adjtimex as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
afs_syscall = libc::SYS_afs_syscall as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
alarm = libc::SYS_alarm as isize,
#[cfg(any(target_arch = "x86_64"))]
arch_prctl = libc::SYS_arch_prctl as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "arm")))]
arm_fadvise64_64 = libc::SYS_arm_fadvise64_64 as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "arm")))]
arm_sync_file_range = libc::SYS_arm_sync_file_range as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
bdflush = libc::SYS_bdflush as isize,
bind = libc::SYS_bind as isize,
bpf = libc::SYS_bpf as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
r#break = libc::SYS_break as isize,
brk = libc::SYS_brk as isize,
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
cachectl = libc::SYS_cachectl as isize,
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
cacheflush = libc::SYS_cacheflush as isize,
capget = libc::SYS_capget as isize,
capset = libc::SYS_capset as isize,
chdir = libc::SYS_chdir as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
chmod = libc::SYS_chmod as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
chown = libc::SYS_chown as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
chown32 = libc::SYS_chown32 as isize,
chroot = libc::SYS_chroot as isize,
clock_adjtime = libc::SYS_clock_adjtime as isize,
clock_getres = libc::SYS_clock_getres as isize,
clock_gettime = libc::SYS_clock_gettime as isize,
clock_nanosleep = libc::SYS_clock_nanosleep as isize,
clock_settime = libc::SYS_clock_settime as isize,
clone = libc::SYS_clone as isize,
close = libc::SYS_close as isize,
connect = libc::SYS_connect as isize,
copy_file_range = libc::SYS_copy_file_range as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
creat = libc::SYS_creat as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
create_module = libc::SYS_create_module as isize,
delete_module = libc::SYS_delete_module as isize,
dup = libc::SYS_dup as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
dup2 = libc::SYS_dup2 as isize,
dup3 = libc::SYS_dup3 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
epoll_create = libc::SYS_epoll_create as isize,
epoll_create1 = libc::SYS_epoll_create1 as isize,
epoll_ctl = libc::SYS_epoll_ctl as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")))), all(target_env = "musl", any(target_arch = "x86_64"))))]
epoll_ctl_old = libc::SYS_epoll_ctl_old as isize,
epoll_pwait = libc::SYS_epoll_pwait as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
epoll_wait = libc::SYS_epoll_wait as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")))), all(target_env = "musl", any(target_arch = "x86_64"))))]
epoll_wait_old = libc::SYS_epoll_wait_old as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
eventfd = libc::SYS_eventfd as isize,
eventfd2 = libc::SYS_eventfd2 as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
execv = libc::SYS_execv as isize,
execve = libc::SYS_execve as isize,
execveat = libc::SYS_execveat as isize,
exit = libc::SYS_exit as isize,
exit_group = libc::SYS_exit_group as isize,
faccessat = libc::SYS_faccessat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips64", target_arch = "powerpc", target_arch = "x86", target_arch = "x86_64"))))]
fadvise64 = libc::SYS_fadvise64 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "powerpc", target_arch = "x86"))))]
fadvise64_64 = libc::SYS_fadvise64_64 as isize,
fallocate = libc::SYS_fallocate as isize,
fanotify_init = libc::SYS_fanotify_init as isize,
fanotify_mark = libc::SYS_fanotify_mark as isize,
fchdir = libc::SYS_fchdir as isize,
fchmod = libc::SYS_fchmod as isize,
fchmodat = libc::SYS_fchmodat as isize,
fchown = libc::SYS_fchown as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
fchown32 = libc::SYS_fchown32 as isize,
fchownat = libc::SYS_fchownat as isize,
fcntl = libc::SYS_fcntl as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))]
fcntl64 = libc::SYS_fcntl64 as isize,
fdatasync = libc::SYS_fdatasync as isize,
fgetxattr = libc::SYS_fgetxattr as isize,
finit_module = libc::SYS_finit_module as isize,
flistxattr = libc::SYS_flistxattr as isize,
flock = libc::SYS_flock as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
fork = libc::SYS_fork as isize,
fremovexattr = libc::SYS_fremovexattr as isize,
fsetxattr = libc::SYS_fsetxattr as isize,
fstat = libc::SYS_fstat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))))]
fstat64 = libc::SYS_fstat64 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "powerpc", target_arch = "x86"))))]
fstatat64 = libc::SYS_fstatat64 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
fstatfs = libc::SYS_fstatfs as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
fstatfs64 = libc::SYS_fstatfs64 as isize,
fsync = libc::SYS_fsync as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
ftime = libc::SYS_ftime as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
ftruncate = libc::SYS_ftruncate as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))]
ftruncate64 = libc::SYS_ftruncate64 as isize,
futex = libc::SYS_futex as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
futimesat = libc::SYS_futimesat as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
get_kernel_syms = libc::SYS_get_kernel_syms as isize,
get_mempolicy = libc::SYS_get_mempolicy as isize,
get_robust_list = libc::SYS_get_robust_list as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "x86")), all(target_env = "musl", any(target_arch = "x86", target_arch = "x86_64"))))]
get_thread_area = libc::SYS_get_thread_area as isize,
getcpu = libc::SYS_getcpu as isize,
getcwd = libc::SYS_getcwd as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
getdents = libc::SYS_getdents as isize,
getdents64 = libc::SYS_getdents64 as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
getdomainname = libc::SYS_getdomainname as isize,
getegid = libc::SYS_getegid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getegid32 = libc::SYS_getegid32 as isize,
geteuid = libc::SYS_geteuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
geteuid32 = libc::SYS_geteuid32 as isize,
getgid = libc::SYS_getgid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getgid32 = libc::SYS_getgid32 as isize,
getgroups = libc::SYS_getgroups as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getgroups32 = libc::SYS_getgroups32 as isize,
getitimer = libc::SYS_getitimer as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
getpagesize = libc::SYS_getpagesize as isize,
getpeername = libc::SYS_getpeername as isize,
getpgid = libc::SYS_getpgid as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
getpgrp = libc::SYS_getpgrp as isize,
getpid = libc::SYS_getpid as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
getpmsg = libc::SYS_getpmsg as isize,
getppid = libc::SYS_getppid as isize,
getpriority = libc::SYS_getpriority as isize,
getrandom = libc::SYS_getrandom as isize,
getresgid = libc::SYS_getresgid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getresgid32 = libc::SYS_getresgid32 as isize,
getresuid = libc::SYS_getresuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getresuid32 = libc::SYS_getresuid32 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
getrlimit = libc::SYS_getrlimit as isize,
getrusage = libc::SYS_getrusage as isize,
getsid = libc::SYS_getsid as isize,
getsockname = libc::SYS_getsockname as isize,
getsockopt = libc::SYS_getsockopt as isize,
gettid = libc::SYS_gettid as isize,
gettimeofday = libc::SYS_gettimeofday as isize,
getuid = libc::SYS_getuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
getuid32 = libc::SYS_getuid32 as isize,
getxattr = libc::SYS_getxattr as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
gtty = libc::SYS_gtty as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
idle = libc::SYS_idle as isize,
init_module = libc::SYS_init_module as isize,
inotify_add_watch = libc::SYS_inotify_add_watch as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
inotify_init = libc::SYS_inotify_init as isize,
inotify_init1 = libc::SYS_inotify_init1 as isize,
inotify_rm_watch = libc::SYS_inotify_rm_watch as isize,
io_cancel = libc::SYS_io_cancel as isize,
io_destroy = libc::SYS_io_destroy as isize,
io_getevents = libc::SYS_io_getevents as isize,
io_setup = libc::SYS_io_setup as isize,
io_submit = libc::SYS_io_submit as isize,
ioctl = libc::SYS_ioctl as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))]
ioperm = libc::SYS_ioperm as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))]
iopl = libc::SYS_iopl as isize,
ioprio_get = libc::SYS_ioprio_get as isize,
ioprio_set = libc::SYS_ioprio_set as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
ipc = libc::SYS_ipc as isize,
kcmp = libc::SYS_kcmp as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
kern_features = libc::SYS_kern_features as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86_64"))]
kexec_file_load = libc::SYS_kexec_file_load as isize,
kexec_load = libc::SYS_kexec_load as isize,
keyctl = libc::SYS_keyctl as isize,
kill = libc::SYS_kill as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
lchown = libc::SYS_lchown as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
lchown32 = libc::SYS_lchown32 as isize,
lgetxattr = libc::SYS_lgetxattr as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
link = libc::SYS_link as isize,
linkat = libc::SYS_linkat as isize,
listen = libc::SYS_listen as isize,
listxattr = libc::SYS_listxattr as isize,
llistxattr = libc::SYS_llistxattr as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
lock = libc::SYS_lock as isize,
lookup_dcookie = libc::SYS_lookup_dcookie as isize,
lremovexattr = libc::SYS_lremovexattr as isize,
lseek = libc::SYS_lseek as isize,
lsetxattr = libc::SYS_lsetxattr as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
lstat = libc::SYS_lstat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))))]
lstat64 = libc::SYS_lstat64 as isize,
madvise = libc::SYS_madvise as isize,
mbind = libc::SYS_mbind as isize,
membarrier = libc::SYS_membarrier as isize,
memfd_create = libc::SYS_memfd_create as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
memory_ordering = libc::SYS_memory_ordering as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
migrate_pages = libc::SYS_migrate_pages as isize,
mincore = libc::SYS_mincore as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
mkdir = libc::SYS_mkdir as isize,
mkdirat = libc::SYS_mkdirat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
mknod = libc::SYS_mknod as isize,
mknodat = libc::SYS_mknodat as isize,
mlock = libc::SYS_mlock as isize,
mlock2 = libc::SYS_mlock2 as isize,
mlockall = libc::SYS_mlockall as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
mmap = libc::SYS_mmap as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))]
mmap2 = libc::SYS_mmap2 as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))]
modify_ldt = libc::SYS_modify_ldt as isize,
mount = libc::SYS_mount as isize,
move_pages = libc::SYS_move_pages as isize,
mprotect = libc::SYS_mprotect as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
mpx = libc::SYS_mpx as isize,
mq_getsetattr = libc::SYS_mq_getsetattr as isize,
mq_notify = libc::SYS_mq_notify as isize,
mq_open = libc::SYS_mq_open as isize,
mq_timedreceive = libc::SYS_mq_timedreceive as isize,
mq_timedsend = libc::SYS_mq_timedsend as isize,
mq_unlink = libc::SYS_mq_unlink as isize,
mremap = libc::SYS_mremap as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
msgctl = libc::SYS_msgctl as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
msgget = libc::SYS_msgget as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
msgrcv = libc::SYS_msgrcv as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
msgsnd = libc::SYS_msgsnd as isize,
msync = libc::SYS_msync as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
multiplexer = libc::SYS_multiplexer as isize,
munlock = libc::SYS_munlock as isize,
munlockall = libc::SYS_munlockall as isize,
munmap = libc::SYS_munmap as isize,
name_to_handle_at = libc::SYS_name_to_handle_at as isize,
nanosleep = libc::SYS_nanosleep as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))))]
newfstatat = libc::SYS_newfstatat as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), target_env = "musl"))]
nfsservctl = libc::SYS_nfsservctl as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
nice = libc::SYS_nice as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
oldfstat = libc::SYS_oldfstat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
oldlstat = libc::SYS_oldlstat as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
oldolduname = libc::SYS_oldolduname as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
oldstat = libc::SYS_oldstat as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
olduname = libc::SYS_olduname as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
open = libc::SYS_open as isize,
open_by_handle_at = libc::SYS_open_by_handle_at as isize,
openat = libc::SYS_openat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
pause = libc::SYS_pause as isize,
#[cfg(any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64"))]
pciconfig_iobase = libc::SYS_pciconfig_iobase as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64"))))]
pciconfig_read = libc::SYS_pciconfig_read as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64"))))]
pciconfig_write = libc::SYS_pciconfig_write as isize,
perf_event_open = libc::SYS_perf_event_open as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
perfctr = libc::SYS_perfctr as isize,
personality = libc::SYS_personality as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
pipe = libc::SYS_pipe as isize,
pipe2 = libc::SYS_pipe2 as isize,
pivot_root = libc::SYS_pivot_root as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "x86", target_arch = "x86_64"))))]
pkey_alloc = libc::SYS_pkey_alloc as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "x86", target_arch = "x86_64"))))]
pkey_free = libc::SYS_pkey_free as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "x86", target_arch = "x86_64"))))]
pkey_mprotect = libc::SYS_pkey_mprotect as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
poll = libc::SYS_poll as isize,
ppoll = libc::SYS_ppoll as isize,
prctl = libc::SYS_prctl as isize,
#[cfg(any(not(target_env = "musl"), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
pread64 = libc::SYS_pread64 as isize,
preadv = libc::SYS_preadv as isize,
preadv2 = libc::SYS_preadv2 as isize,
prlimit64 = libc::SYS_prlimit64 as isize,
process_vm_readv = libc::SYS_process_vm_readv as isize,
process_vm_writev = libc::SYS_process_vm_writev as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
prof = libc::SYS_prof as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
profil = libc::SYS_profil as isize,
pselect6 = libc::SYS_pselect6 as isize,
ptrace = libc::SYS_ptrace as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
putpmsg = libc::SYS_putpmsg as isize,
#[cfg(any(not(target_env = "musl"), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
pwrite64 = libc::SYS_pwrite64 as isize,
pwritev = libc::SYS_pwritev as isize,
pwritev2 = libc::SYS_pwritev2 as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
query_module = libc::SYS_query_module as isize,
quotactl = libc::SYS_quotactl as isize,
read = libc::SYS_read as isize,
readahead = libc::SYS_readahead as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
readdir = libc::SYS_readdir as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
readlink = libc::SYS_readlink as isize,
readlinkat = libc::SYS_readlinkat as isize,
readv = libc::SYS_readv as isize,
reboot = libc::SYS_reboot as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64"))]
recv = libc::SYS_recv as isize,
recvfrom = libc::SYS_recvfrom as isize,
recvmmsg = libc::SYS_recvmmsg as isize,
recvmsg = libc::SYS_recvmsg as isize,
remap_file_pages = libc::SYS_remap_file_pages as isize,
removexattr = libc::SYS_removexattr as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
rename = libc::SYS_rename as isize,
renameat = libc::SYS_renameat as isize,
renameat2 = libc::SYS_renameat2 as isize,
request_key = libc::SYS_request_key as isize,
restart_syscall = libc::SYS_restart_syscall as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
rmdir = libc::SYS_rmdir as isize,
rt_sigaction = libc::SYS_rt_sigaction as isize,
rt_sigpending = libc::SYS_rt_sigpending as isize,
rt_sigprocmask = libc::SYS_rt_sigprocmask as isize,
rt_sigqueueinfo = libc::SYS_rt_sigqueueinfo as isize,
rt_sigreturn = libc::SYS_rt_sigreturn as isize,
rt_sigsuspend = libc::SYS_rt_sigsuspend as isize,
rt_sigtimedwait = libc::SYS_rt_sigtimedwait as isize,
rt_tgsigqueueinfo = libc::SYS_rt_tgsigqueueinfo as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
rtas = libc::SYS_rtas as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "s390x")))]
s390_pci_mmio_read = libc::SYS_s390_pci_mmio_read as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "s390x")))]
s390_pci_mmio_write = libc::SYS_s390_pci_mmio_write as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "s390x")))]
s390_runtime_instr = libc::SYS_s390_runtime_instr as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
sched_get_affinity = libc::SYS_sched_get_affinity as isize,
sched_get_priority_max = libc::SYS_sched_get_priority_max as isize,
sched_get_priority_min = libc::SYS_sched_get_priority_min as isize,
sched_getaffinity = libc::SYS_sched_getaffinity as isize,
sched_getattr = libc::SYS_sched_getattr as isize,
sched_getparam = libc::SYS_sched_getparam as isize,
sched_getscheduler = libc::SYS_sched_getscheduler as isize,
sched_rr_get_interval = libc::SYS_sched_rr_get_interval as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
sched_set_affinity = libc::SYS_sched_set_affinity as isize,
sched_setaffinity = libc::SYS_sched_setaffinity as isize,
sched_setattr = libc::SYS_sched_setattr as isize,
sched_setparam = libc::SYS_sched_setparam as isize,
sched_setscheduler = libc::SYS_sched_setscheduler as isize,
sched_yield = libc::SYS_sched_yield as isize,
seccomp = libc::SYS_seccomp as isize,
#[cfg(any(target_arch = "x86_64"))]
security = libc::SYS_security as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
select = libc::SYS_select as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
semctl = libc::SYS_semctl as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
semget = libc::SYS_semget as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
semop = libc::SYS_semop as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
semtimedop = libc::SYS_semtimedop as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64"))]
send = libc::SYS_send as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
sendfile = libc::SYS_sendfile as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))))]
sendfile64 = libc::SYS_sendfile64 as isize,
sendmmsg = libc::SYS_sendmmsg as isize,
sendmsg = libc::SYS_sendmsg as isize,
sendto = libc::SYS_sendto as isize,
set_mempolicy = libc::SYS_set_mempolicy as isize,
set_robust_list = libc::SYS_set_robust_list as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "mips", target_arch = "mips64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64"))))]
set_thread_area = libc::SYS_set_thread_area as isize,
set_tid_address = libc::SYS_set_tid_address as isize,
setdomainname = libc::SYS_setdomainname as isize,
setfsgid = libc::SYS_setfsgid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setfsgid32 = libc::SYS_setfsgid32 as isize,
setfsuid = libc::SYS_setfsuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setfsuid32 = libc::SYS_setfsuid32 as isize,
setgid = libc::SYS_setgid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setgid32 = libc::SYS_setgid32 as isize,
setgroups = libc::SYS_setgroups as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setgroups32 = libc::SYS_setgroups32 as isize,
sethostname = libc::SYS_sethostname as isize,
setitimer = libc::SYS_setitimer as isize,
setns = libc::SYS_setns as isize,
setpgid = libc::SYS_setpgid as isize,
setpriority = libc::SYS_setpriority as isize,
setregid = libc::SYS_setregid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setregid32 = libc::SYS_setregid32 as isize,
setresgid = libc::SYS_setresgid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setresgid32 = libc::SYS_setresgid32 as isize,
setresuid = libc::SYS_setresuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setresuid32 = libc::SYS_setresuid32 as isize,
setreuid = libc::SYS_setreuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setreuid32 = libc::SYS_setreuid32 as isize,
setrlimit = libc::SYS_setrlimit as isize,
setsid = libc::SYS_setsid as isize,
setsockopt = libc::SYS_setsockopt as isize,
settimeofday = libc::SYS_settimeofday as isize,
setuid = libc::SYS_setuid as isize,
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
setuid32 = libc::SYS_setuid32 as isize,
setxattr = libc::SYS_setxattr as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sgetmask = libc::SYS_sgetmask as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
shmat = libc::SYS_shmat as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
shmctl = libc::SYS_shmctl as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
shmdt = libc::SYS_shmdt as isize,
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips64", target_arch = "x86_64"))]
shmget = libc::SYS_shmget as isize,
shutdown = libc::SYS_shutdown as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sigaction = libc::SYS_sigaction as isize,
sigaltstack = libc::SYS_sigaltstack as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
signal = libc::SYS_signal as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
signalfd = libc::SYS_signalfd as isize,
signalfd4 = libc::SYS_signalfd4 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sigpending = libc::SYS_sigpending as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sigprocmask = libc::SYS_sigprocmask as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sigreturn = libc::SYS_sigreturn as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
sigsuspend = libc::SYS_sigsuspend as isize,
socket = libc::SYS_socket as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
socketcall = libc::SYS_socketcall as isize,
socketpair = libc::SYS_socketpair as isize,
splice = libc::SYS_splice as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
spu_create = libc::SYS_spu_create as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
spu_run = libc::SYS_spu_run as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
ssetmask = libc::SYS_ssetmask as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
stat = libc::SYS_stat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))))]
stat64 = libc::SYS_stat64 as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
statfs = libc::SYS_statfs as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
statfs64 = libc::SYS_statfs64 as isize,
statx = libc::SYS_statx as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
stime = libc::SYS_stime as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
stty = libc::SYS_stty as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
subpage_prot = libc::SYS_subpage_prot as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
swapcontext = libc::SYS_swapcontext as isize,
swapoff = libc::SYS_swapoff as isize,
swapon = libc::SYS_swapon as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
switch_endian = libc::SYS_switch_endian as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
symlink = libc::SYS_symlink as isize,
symlinkat = libc::SYS_symlinkat as isize,
sync = libc::SYS_sync as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64"))))]
sync_file_range = libc::SYS_sync_file_range as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
sync_file_range2 = libc::SYS_sync_file_range2 as isize,
syncfs = libc::SYS_syncfs as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
sys_debug_setcontext = libc::SYS_sys_debug_setcontext as isize,
#[cfg(any(target_arch = "mips"))]
syscall = libc::SYS_syscall as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
sysfs = libc::SYS_sysfs as isize,
sysinfo = libc::SYS_sysinfo as isize,
syslog = libc::SYS_syslog as isize,
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
sysmips = libc::SYS_sysmips as isize,
tee = libc::SYS_tee as isize,
tgkill = libc::SYS_tgkill as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))]
time = libc::SYS_time as isize,
timer_create = libc::SYS_timer_create as isize,
timer_delete = libc::SYS_timer_delete as isize,
timer_getoverrun = libc::SYS_timer_getoverrun as isize,
timer_gettime = libc::SYS_timer_gettime as isize,
timer_settime = libc::SYS_timer_settime as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "s390x")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64"))))]
timerfd = libc::SYS_timerfd as isize,
timerfd_create = libc::SYS_timerfd_create as isize,
timerfd_gettime = libc::SYS_timerfd_gettime as isize,
timerfd_settime = libc::SYS_timerfd_settime as isize,
times = libc::SYS_times as isize,
tkill = libc::SYS_tkill as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
truncate = libc::SYS_truncate as isize,
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "x86"))]
truncate64 = libc::SYS_truncate64 as isize,
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86_64"))]
tuxcall = libc::SYS_tuxcall as isize,
#[cfg(any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
ugetrlimit = libc::SYS_ugetrlimit as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
ulimit = libc::SYS_ulimit as isize,
umask = libc::SYS_umask as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
umount = libc::SYS_umount as isize,
umount2 = libc::SYS_umount2 as isize,
uname = libc::SYS_uname as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
unlink = libc::SYS_unlink as isize,
unlinkat = libc::SYS_unlinkat as isize,
unshare = libc::SYS_unshare as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
uselib = libc::SYS_uselib as isize,
userfaultfd = libc::SYS_userfaultfd as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
ustat = libc::SYS_ustat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
utime = libc::SYS_utime as isize,
utimensat = libc::SYS_utimensat as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
utimes = libc::SYS_utimes as isize,
#[cfg(all(not(target_env = "musl"), any(target_arch = "sparc64")))]
utrap_install = libc::SYS_utrap_install as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64")), all(target_env = "musl", any(target_arch = "arm", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64"))))]
vfork = libc::SYS_vfork as isize,
vhangup = libc::SYS_vhangup as isize,
#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))]
vm86 = libc::SYS_vm86 as isize,
#[cfg(any(target_arch = "x86"))]
vm86old = libc::SYS_vm86old as isize,
vmsplice = libc::SYS_vmsplice as isize,
#[cfg(any(all(not(target_env = "musl"), any(all(target_arch = "x86_64", not(target_pointer_width = "32")), target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "x86", target_arch = "x86_64"))))]
vserver = libc::SYS_vserver as isize,
wait4 = libc::SYS_wait4 as isize,
waitid = libc::SYS_waitid as isize,
#[cfg(any(all(not(target_env = "musl"), any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64", target_arch = "x86")), all(target_env = "musl", any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86"))))]
waitpid = libc::SYS_waitpid as isize,
write = libc::SYS_write as isize,
writev = libc::SYS_writev as isize,
}

impl Syscall {
    #[inline]
    pub fn into_i32(self) -> i32 {
        self as i32
    }

    #[inline]
    pub fn from_name(name: &str) -> Option<Self>{
        use std::str::FromStr;
        Self::from_str(name).ok()
    }
}
