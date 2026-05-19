use crate::Ins;

impl Ins {
    pub fn discriminant(&self) -> u16 {
        unsafe { *(self as *const Self as *const u16) }
    }

    /// Returns `true` for any instruction that may transfer control flow,
    /// whether or not it has a delay slot.
    pub fn is_branch(&self) -> bool {
        match self {
            Ins::Bt { .. }
            | Ins::Bf { .. }
            | Ins::Bra { .. }
            | Ins::Bsr { .. }
            | Ins::JmpAtRn { .. }
            | Ins::JsrAtRn { .. }
            | Ins::Rts
            | Ins::Rte => true,
            #[cfg(feature = "sh2")]
            Ins::BrafRn { .. } | Ins::BsrfRn { .. } | Ins::Bts { .. } | Ins::Bfs { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this branch instruction executes the following instruction
    /// (the delay slot) before transferring control.
    pub fn is_delayed_branch(&self) -> bool {
        match self {
            Ins::Bra { .. }
            | Ins::Bsr { .. }
            | Ins::JmpAtRn { .. }
            | Ins::JsrAtRn { .. }
            | Ins::Rts
            | Ins::Rte => true,
            #[cfg(feature = "sh2")]
            Ins::BrafRn { .. } | Ins::BsrfRn { .. } | Ins::Bts { .. } | Ins::Bfs { .. } => true,
            _ => false,
        }
    }
}
