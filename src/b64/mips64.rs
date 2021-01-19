#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Syscall {
read = 0,
write = 1,
open = 2,
close = 3,
stat = 4,
fstat = 5,
lstat = 6,
poll = 7,
lseek = 8,
mmap = 9,
mprotect = 10,
munmap = 11,
brk = 12,
rt_sigaction = 13,
rt_sigprocmask = 14,
ioctl = 15,
pread64 = 16,
pwrite64 = 17,
readv = 18,
writev = 19,
access = 20,
pipe = 21,
_newselect = 22,
sched_yield = 23,
mremap = 24,
msync = 25,
mincore = 26,
madvise = 27,
shmget = 28,
shmat = 29,
shmctl = 30,
dup = 31,
dup2 = 32,
pause = 33,
nanosleep = 34,
getitimer = 35,
setitimer = 36,
alarm = 37,
getpid = 38,
sendfile = 39,
socket = 40,
connect = 41,
accept = 42,
sendto = 43,
recvfrom = 44,
sendmsg = 45,
recvmsg = 46,
shutdown = 47,
bind = 48,
listen = 49,
getsockname = 50,
getpeername = 51,
socketpair = 52,
setsockopt = 53,
getsockopt = 54,
clone = 55,
fork = 56,
execve = 57,
exit = 58,
wait4 = 59,
kill = 60,
uname = 61,
semget = 62,
semop = 63,
semctl = 64,
shmdt = 65,
msgget = 66,
msgsnd = 67,
msgrcv = 68,
msgctl = 69,
fcntl = 70,
flock = 71,
fsync = 72,
fdatasync = 73,
truncate = 74,
ftruncate = 75,
getdents = 76,
getcwd = 77,
chdir = 78,
fchdir = 79,
rename = 80,
mkdir = 81,
rmdir = 82,
creat = 83,
link = 84,
unlink = 85,
symlink = 86,
readlink = 87,
chmod = 88,
fchmod = 89,
chown = 90,
fchown = 91,
lchown = 92,
umask = 93,
gettimeofday = 94,
getrlimit = 95,
getrusage = 96,
sysinfo = 97,
times = 98,
ptrace = 99,
getuid = 100,
syslog = 101,
getgid = 102,
setuid = 103,
setgid = 104,
geteuid = 105,
getegid = 106,
setpgid = 107,
getppid = 108,
getpgrp = 109,
setsid = 110,
setreuid = 111,
setregid = 112,
getgroups = 113,
setgroups = 114,
setresuid = 115,
getresuid = 116,
setresgid = 117,
getresgid = 118,
getpgid = 119,
setfsuid = 120,
setfsgid = 121,
getsid = 122,
capget = 123,
capset = 124,
rt_sigpending = 125,
rt_sigtimedwait = 126,
rt_sigqueueinfo = 127,
rt_sigsuspend = 128,
sigaltstack = 129,
utime = 130,
mknod = 131,
personality = 132,
ustat = 133,
statfs = 134,
fstatfs = 135,
sysfs = 136,
getpriority = 137,
setpriority = 138,
sched_setparam = 139,
sched_getparam = 140,
sched_setscheduler = 141,
sched_getscheduler = 142,
sched_get_priority_max = 143,
sched_get_priority_min = 144,
sched_rr_get_interval = 145,
mlock = 146,
munlock = 147,
mlockall = 148,
munlockall = 149,
vhangup = 150,
pivot_root = 151,
_sysctl = 152,
prctl = 153,
adjtimex = 154,
setrlimit = 155,
chroot = 156,
sync = 157,
acct = 158,
settimeofday = 159,
mount = 160,
umount2 = 161,
swapon = 162,
swapoff = 163,
reboot = 164,
sethostname = 165,
setdomainname = 166,
create_module = 167,
init_module = 168,
delete_module = 169,
get_kernel_syms = 170,
query_module = 171,
quotactl = 172,
nfsservctl = 173,
getpmsg = 174,
putpmsg = 175,
afs_syscall = 176,
reserved177 = 177,
gettid = 178,
readahead = 179,
setxattr = 180,
lsetxattr = 181,
fsetxattr = 182,
getxattr = 183,
lgetxattr = 184,
fgetxattr = 185,
listxattr = 186,
llistxattr = 187,
flistxattr = 188,
removexattr = 189,
lremovexattr = 190,
fremovexattr = 191,
tkill = 192,
reserved193 = 193,
futex = 194,
sched_setaffinity = 195,
sched_getaffinity = 196,
cacheflush = 197,
cachectl = 198,
sysmips = 199,
io_setup = 200,
io_destroy = 201,
io_getevents = 202,
io_submit = 203,
io_cancel = 204,
exit_group = 205,
lookup_dcookie = 206,
epoll_create = 207,
epoll_ctl = 208,
epoll_wait = 209,
remap_file_pages = 210,
rt_sigreturn = 211,
set_tid_address = 212,
restart_syscall = 213,
semtimedop = 214,
fadvise64 = 215,
timer_create = 216,
timer_settime = 217,
timer_gettime = 218,
timer_getoverrun = 219,
timer_delete = 220,
clock_settime = 221,
clock_gettime = 222,
clock_getres = 223,
clock_nanosleep = 224,
tgkill = 225,
utimes = 226,
mbind = 227,
get_mempolicy = 228,
set_mempolicy = 229,
mq_open = 230,
mq_unlink = 231,
mq_timedsend = 232,
mq_timedreceive = 233,
mq_notify = 234,
mq_getsetattr = 235,
vserver = 236,
waitid = 237,
add_key = 239,
request_key = 240,
keyctl = 241,
set_thread_area = 242,
inotify_init = 243,
inotify_add_watch = 244,
inotify_rm_watch = 245,
migrate_pages = 246,
openat = 247,
mkdirat = 248,
mknodat = 249,
fchownat = 250,
futimesat = 251,
newfstatat = 252,
unlinkat = 253,
renameat = 254,
linkat = 255,
symlinkat = 256,
readlinkat = 257,
fchmodat = 258,
faccessat = 259,
pselect6 = 260,
ppoll = 261,
unshare = 262,
splice = 263,
sync_file_range = 264,
tee = 265,
vmsplice = 266,
move_pages = 267,
set_robust_list = 268,
get_robust_list = 269,
kexec_load = 270,
getcpu = 271,
epoll_pwait = 272,
ioprio_set = 273,
ioprio_get = 274,
utimensat = 275,
signalfd = 276,
timerfd = 277,
eventfd = 278,
fallocate = 279,
timerfd_create = 280,
timerfd_gettime = 281,
timerfd_settime = 282,
signalfd4 = 283,
eventfd2 = 284,
epoll_create1 = 285,
dup3 = 286,
pipe2 = 287,
inotify_init1 = 288,
preadv = 289,
pwritev = 290,
rt_tgsigqueueinfo = 291,
perf_event_open = 292,
accept4 = 293,
recvmmsg = 294,
fanotify_init = 295,
fanotify_mark = 296,
prlimit64 = 297,
name_to_handle_at = 298,
open_by_handle_at = 299,
clock_adjtime = 300,
syncfs = 301,
sendmmsg = 302,
setns = 303,
process_vm_readv = 304,
process_vm_writev = 305,
kcmp = 306,
finit_module = 307,
getdents64 = 308,
sched_setattr = 309,
sched_getattr = 310,
renameat2 = 311,
seccomp = 312,
getrandom = 313,
memfd_create = 314,
bpf = 315,
execveat = 316,
userfaultfd = 317,
membarrier = 318,
mlock2 = 319,
copy_file_range = 320,
preadv2 = 321,
pwritev2 = 322,
pkey_mprotect = 323,
pkey_alloc = 324,
pkey_free = 325,
statx = 326,
rseq = 327,
io_pgetevents = 328,
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
}
