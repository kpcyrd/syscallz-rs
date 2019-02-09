#!/usr/bin/env python
import re
import sys

X86_64_NOT_32   = 'all(target_arch = "x86_64", not(target_pointer_width = "32"))'
X86_64_32       = 'all(target_arch = "x86_64", target_pointer_width = "32")'

BASE = (sys.argv[1] if len(sys.argv) > 1 else '.') + '/src/unix/notbsd/linux'
ARCHS = {
    'target_arch = "mips"': 'mips/mips32.rs',
    'target_arch = "mips64"': 'mips/mips64.rs',

    'target_arch = "arm"': 'other/b32/arm.rs',
    'target_arch = "powerpc"': 'other/b32/powerpc.rs',
    'target_arch = "x86"': 'other/b32/x86.rs',

    'target_arch = "aarch64"': 'other/b64/aarch64.rs',
    X86_64_NOT_32: 'other/b64/not_x32.rs',
    'target_arch = "powerpc64"': 'other/b64/powerpc64.rs',
    'target_arch = "sparc64"': 'other/b64/sparc64.rs',
    X86_64_32: 'other/b64/x32.rs',
    'target_arch = "s390x"': 's390x.rs',
}
# target_env = "musl"
MUSL_ARCHS = {
    'target_arch = "arm"': 'musl/b32/arm.rs',
    'target_arch = "mips"': 'musl/b32/mips.rs',
    'target_arch = "powerpc"': 'musl/b32/powerpc.rs',
    'target_arch = "x86"': 'musl/b32/x86.rs',
    'target_arch = "aarch64"': 'musl/b64/aarch64.rs',
    'target_arch = "powerpc64"': 'musl/b64/powerpc64.rs',
    'target_arch = "x86_64"': 'musl/b64/x86_64.rs',
}

def find_syscalls(archs):
    SYSCALLS = {}
    REGEX = re.compile('^pub const SYS_([^:]+)')
    for arch, path in archs.items():
        with open('%s/%s' % (BASE, path)) as f:
            for line in f:
                m = REGEX.match(line)
                if m:
                    sc = m.group(1)
                    if sc not in SYSCALLS:
                        SYSCALLS[sc] = set()
                    SYSCALLS[sc].add(arch)
    return SYSCALLS

SYSCALLS = find_syscalls(ARCHS)
MUSL_SYSCALLS = find_syscalls(MUSL_ARCHS)

def gen_conditions(conditions, archs):
    if len(conditions) < len(archs.values()):
        if X86_64_NOT_32 in conditions and X86_64_32 in conditions:
            conditions.remove(X86_64_NOT_32)
            conditions.remove(X86_64_32)
            conditions.add('target_arch = "x86_64"')
        conditions = list(conditions)
        conditions.sort()
        return 'any(' + ', '.join(conditions) + ')'

print('''use libc;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Syscall {''')

syscalls = list(set(SYSCALLS.keys()) | set(MUSL_SYSCALLS.keys()))
syscalls.sort()
for sc in syscalls:
    conditions = []

    not_musl_conditions = SYSCALLS.get(sc)
    if not_musl_conditions:
        not_musl_conditions = gen_conditions(not_musl_conditions, ARCHS)
        x = 'not(target_env = "musl")'
        if not_musl_conditions:
            x = 'all(' + x + ', ' + not_musl_conditions + ')'
        conditions.append(x)

    musl_conditions = MUSL_SYSCALLS.get(sc)
    if musl_conditions:
        musl_conditions = gen_conditions(musl_conditions, MUSL_ARCHS)
        x = 'target_env = "musl"'
        if musl_conditions:
            x = 'all(' + x + ', ' + musl_conditions + ')'
        conditions.append(x)

    # if musl and not_musl are identical, don't check for target_env
    if not_musl_conditions and not_musl_conditions == musl_conditions:
        conditions = [not_musl_conditions]

    # if both target_envs have no arch condtions, fully drop all conditions
    if not not_musl_conditions and not musl_conditions:
        conditions = []

    # stringify conditions:
    if conditions:
        if len(conditions) == 1:
            conditions = conditions[0]
        else:
            conditions = 'any(' + ', '.join(conditions) + ')'
        print('#[cfg(' + conditions + ')]')

    name = ('r#' + sc) if sc in ['break'] else sc
    print('%s = libc::SYS_%s as isize,' % (name, sc))

print('''}

impl Syscall {
    #[inline]
    pub fn into_i32(self) -> i32 {
        self as i32
    }
}''')
