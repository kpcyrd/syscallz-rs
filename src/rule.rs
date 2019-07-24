use scmp_compare::*;
use seccomp_sys::{scmp_arg_cmp, scmp_compare};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Cmp {
    Ne,
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
    MaskedEq,
}

impl Cmp {
    #[inline]
    fn force_from(compare: scmp_compare) -> Self {
        use self::Cmp::*;
        match compare {
            SCMP_CMP_NE => Ne,
            SCMP_CMP_LT => Lt,
            SCMP_CMP_LE => Le,
            SCMP_CMP_EQ => Eq,
            SCMP_CMP_GE => Ge,
            SCMP_CMP_GT => Gt,
            SCMP_CMP_MASKED_EQ => MaskedEq,
            _ => unreachable!(),
        }
    }
}

impl Into<scmp_compare> for Cmp {
    fn into(self) -> scmp_compare {
        use self::Cmp::*;
        match self {
            Ne => SCMP_CMP_NE,
            Lt => SCMP_CMP_LT,
            Le => SCMP_CMP_LE,
            Eq => SCMP_CMP_EQ,
            Ge => SCMP_CMP_GE,
            Gt => SCMP_CMP_GT,
            MaskedEq => SCMP_CMP_MASKED_EQ,
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Comparator(scmp_arg_cmp);

impl Clone for Comparator {
    fn clone(&self) -> Self {
        unsafe { std::ptr::read(self as *const Comparator) }
    }
}

impl Comparator {
    pub fn new(arg: u32, op: Cmp, datum_a: u64, datum_b: u64) -> Self {
        Self(scmp_arg_cmp {
            arg,
            op: op.into(),
            datum_a,
            datum_b,
        })
    }

    #[inline]
    pub fn arg(&self) -> u32 {
        self.0.arg
    }

    #[inline]
    pub fn op(&self) -> Cmp {
        Cmp::force_from(self.0.op)
    }

    #[inline]
    pub fn datum_a(&self) -> u64 {
        self.0.datum_a
    }

    #[inline]
    pub fn datum_b(&self) -> u64 {
        self.0.datum_b
    }
}
