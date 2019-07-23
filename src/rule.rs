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

impl Into<scmp_compare> for Cmp {
    fn into(self) -> scmp_compare {
        use self::Cmp::*;
        use scmp_compare::*;
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
    pub fn new(arg: u32, op: Cmp, datum_a: u64, datum_b: Option<u64>) -> Self {
        Self(scmp_arg_cmp {
            arg,
            op: op.into(),
            datum_a,
            datum_b: datum_b.unwrap_or(0_u64),
        })
    }
}
