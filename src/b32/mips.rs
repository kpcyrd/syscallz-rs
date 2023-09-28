/// An enum of all syscalls
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Syscall {
syscall = 0,
exit = 1,
fork = 2,
read = 3,
write = 4,
open = 5,
close = 6,
waitpid = 7,
creat = 8,
link = 9,
unlink = 10,
execve = 11,
chdir = 12,
time = 13,
mknod = 14,
chmod = 15,
lchown = 16,
_break = 17,
unused18 = 18,
lseek = 19,
getpid = 20,
mount = 21,
umount = 22,
setuid = 23,
getuid = 24,
stime = 25,
ptrace = 26,
alarm = 27,
unused28 = 28,
pause = 29,
utime = 30,
stty = 31,
gtty = 32,
access = 33,
nice = 34,
ftime = 35,
sync = 36,
kill = 37,
rename = 38,
mkdir = 39,
rmdir = 40,
dup = 41,
pipe = 42,
times = 43,
prof = 44,
brk = 45,
setgid = 46,
getgid = 47,
signal = 48,
geteuid = 49,
getegid = 50,
acct = 51,
umount2 = 52,
lock = 53,
ioctl = 54,
fcntl = 55,
mpx = 56,
setpgid = 57,
ulimit = 58,
unused59 = 59,
umask = 60,
chroot = 61,
ustat = 62,
dup2 = 63,
getppid = 64,
getpgrp = 65,
setsid = 66,
sigaction = 67,
sgetmask = 68,
ssetmask = 69,
setreuid = 70,
setregid = 71,
sigsuspend = 72,
sigpending = 73,
sethostname = 74,
setrlimit = 75,
getrlimit = 76,
getrusage = 77,
gettimeofday = 78,
settimeofday = 79,
getgroups = 80,
setgroups = 81,
reserved82 = 82,
symlink = 83,
unused84 = 84,
readlink = 85,
uselib = 86,
swapon = 87,
reboot = 88,
readdir = 89,
mmap = 90,
munmap = 91,
truncate = 92,
ftruncate = 93,
fchmod = 94,
fchown = 95,
getpriority = 96,
setpriority = 97,
profil = 98,
statfs = 99,
fstatfs = 100,
ioperm = 101,
socketcall = 102,
syslog = 103,
setitimer = 104,
getitimer = 105,
stat = 106,
lstat = 107,
fstat = 108,
unused109 = 109,
iopl = 110,
vhangup = 111,
idle = 112,
vm86 = 113,
wait4 = 114,
swapoff = 115,
sysinfo = 116,
ipc = 117,
fsync = 118,
sigreturn = 119,
clone = 120,
setdomainname = 121,
uname = 122,
modify_ldt = 123,
adjtimex = 124,
mprotect = 125,
sigprocmask = 126,
create_module = 127,
init_module = 128,
delete_module = 129,
get_kernel_syms = 130,
quotactl = 131,
getpgid = 132,
fchdir = 133,
bdflush = 134,
sysfs = 135,
personality = 136,
afs_syscall = 137,
setfsuid = 138,
setfsgid = 139,
_llseek = 140,
getdents = 141,
_newselect = 142,
flock = 143,
msync = 144,
readv = 145,
writev = 146,
cacheflush = 147,
cachectl = 148,
sysmips = 149,
unused150 = 150,
getsid = 151,
fdatasync = 152,
_sysctl = 153,
mlock = 154,
munlock = 155,
mlockall = 156,
munlockall = 157,
sched_setparam = 158,
sched_getparam = 159,
sched_setscheduler = 160,
sched_getscheduler = 161,
sched_yield = 162,
sched_get_priority_max = 163,
sched_get_priority_min = 164,
sched_rr_get_interval = 165,
nanosleep = 166,
mremap = 167,
accept = 168,
bind = 169,
connect = 170,
getpeername = 171,
getsockname = 172,
getsockopt = 173,
listen = 174,
recv = 175,
recvfrom = 176,
recvmsg = 177,
send = 178,
sendmsg = 179,
sendto = 180,
setsockopt = 181,
shutdown = 182,
socket = 183,
socketpair = 184,
setresuid = 185,
getresuid = 186,
query_module = 187,
poll = 188,
nfsservctl = 189,
setresgid = 190,
getresgid = 191,
prctl = 192,
rt_sigreturn = 193,
rt_sigaction = 194,
rt_sigprocmask = 195,
rt_sigpending = 196,
rt_sigtimedwait = 197,
rt_sigqueueinfo = 198,
rt_sigsuspend = 199,
pread64 = 200,
pwrite64 = 201,
chown = 202,
getcwd = 203,
capget = 204,
capset = 205,
sigaltstack = 206,
sendfile = 207,
getpmsg = 208,
putpmsg = 209,
mmap2 = 210,
truncate64 = 211,
ftruncate64 = 212,
stat64 = 213,
lstat64 = 214,
fstat64 = 215,
pivot_root = 216,
mincore = 217,
madvise = 218,
getdents64 = 219,
fcntl64 = 220,
reserved221 = 221,
gettid = 222,
readahead = 223,
setxattr = 224,
lsetxattr = 225,
fsetxattr = 226,
getxattr = 227,
lgetxattr = 228,
fgetxattr = 229,
listxattr = 230,
llistxattr = 231,
flistxattr = 232,
removexattr = 233,
lremovexattr = 234,
fremovexattr = 235,
tkill = 236,
sendfile64 = 237,
futex = 238,
sched_setaffinity = 239,
sched_getaffinity = 240,
io_setup = 241,
io_destroy = 242,
io_getevents = 243,
io_submit = 244,
io_cancel = 245,
exit_group = 246,
lookup_dcookie = 247,
epoll_create = 248,
epoll_ctl = 249,
epoll_wait = 250,
remap_file_pages = 251,
set_tid_address = 252,
restart_syscall = 253,
fadvise64 = 254,
statfs64 = 255,
fstatfs64 = 256,
timer_create = 257,
timer_settime = 258,
timer_gettime = 259,
timer_getoverrun = 260,
timer_delete = 261,
clock_settime = 262,
clock_gettime = 263,
clock_getres = 264,
clock_nanosleep = 265,
tgkill = 266,
utimes = 267,
mbind = 268,
get_mempolicy = 269,
set_mempolicy = 270,
mq_open = 271,
mq_unlink = 272,
mq_timedsend = 273,
mq_timedreceive = 274,
mq_notify = 275,
mq_getsetattr = 276,
vserver = 277,
waitid = 278,
add_key = 280,
request_key = 281,
keyctl = 282,
set_thread_area = 283,
inotify_init = 284,
inotify_add_watch = 285,
inotify_rm_watch = 286,
migrate_pages = 287,
openat = 288,
mkdirat = 289,
mknodat = 290,
fchownat = 291,
futimesat = 292,
fstatat64 = 293,
unlinkat = 294,
renameat = 295,
linkat = 296,
symlinkat = 297,
readlinkat = 298,
fchmodat = 299,
faccessat = 300,
pselect6 = 301,
ppoll = 302,
unshare = 303,
splice = 304,
sync_file_range = 305,
tee = 306,
vmsplice = 307,
move_pages = 308,
set_robust_list = 309,
get_robust_list = 310,
kexec_load = 311,
getcpu = 312,
epoll_pwait = 313,
ioprio_set = 314,
ioprio_get = 315,
utimensat = 316,
signalfd = 317,
timerfd = 318,
eventfd = 319,
fallocate = 320,
timerfd_create = 321,
timerfd_gettime = 322,
timerfd_settime = 323,
signalfd4 = 324,
eventfd2 = 325,
epoll_create1 = 326,
dup3 = 327,
pipe2 = 328,
inotify_init1 = 329,
preadv = 330,
pwritev = 331,
rt_tgsigqueueinfo = 332,
perf_event_open = 333,
accept4 = 334,
recvmmsg = 335,
fanotify_init = 336,
fanotify_mark = 337,
prlimit64 = 338,
name_to_handle_at = 339,
open_by_handle_at = 340,
clock_adjtime = 341,
syncfs = 342,
sendmmsg = 343,
setns = 344,
process_vm_readv = 345,
process_vm_writev = 346,
kcmp = 347,
finit_module = 348,
sched_setattr = 349,
sched_getattr = 350,
renameat2 = 351,
seccomp = 352,
getrandom = 353,
memfd_create = 354,
bpf = 355,
execveat = 356,
userfaultfd = 357,
membarrier = 358,
mlock2 = 359,
copy_file_range = 360,
preadv2 = 361,
pwritev2 = 362,
pkey_mprotect = 363,
pkey_alloc = 364,
pkey_free = 365,
statx = 366,
rseq = 367,
io_pgetevents = 368,
semget = 393,
semctl = 394,
shmget = 395,
shmctl = 396,
shmat = 397,
shmdt = 398,
msgget = 399,
msgsnd = 400,
msgrcv = 401,
msgctl = 402,
clock_gettime64 = 403,
clock_settime64 = 404,
clock_adjtime64 = 405,
clock_getres_time64 = 406,
clock_nanosleep_time64 = 407,
timer_gettime64 = 408,
timer_settime64 = 409,
timerfd_gettime64 = 410,
timerfd_settime64 = 411,
utimensat_time64 = 412,
pselect6_time64 = 413,
ppoll_time64 = 414,
io_pgetevents_time64 = 416,
recvmmsg_time64 = 417,
mq_timedsend_time64 = 418,
mq_timedreceive_time64 = 419,
semtimedop_time64 = 420,
rt_sigtimedwait_time64 = 421,
futex_time64 = 422,
sched_rr_get_interval_time64 = 423,
pidfd_send_signal = 424,
io_uring_setup = 425,
io_uring_enter = 426,
io_uring_register = 427,
open_tree = 428,
move_mount = 429,
fsopen = 430,
fsconfig = 431,
fsmount = 432,
fspick = 433,
pidfd_open = 434,
clone3 = 435,
close_range = 436,
openat2 = 437,
pidfd_getfd = 438,
faccessat2 = 439,
process_madvise = 440,
epoll_pwait2 = 441,
mount_setattr = 442,
quotactl_fd = 443,
landlock_create_ruleset = 444,
landlock_add_rule = 445,
landlock_restrict_self = 446,
process_mrelease = 448,
futex_waitv = 449,
set_mempolicy_home_node = 450,
cachestat = 451,
}
