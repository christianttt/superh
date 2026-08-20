use crate::{Address, ControlFlow, Ins};

/// A borrowed instruction associated with its memory address.
#[must_use = "located instructions carry address-dependent information"]
#[derive(Clone, Copy, Debug)]
pub struct LocatedIns<'a> {
    instruction: &'a Ins,
    address: u32,
}

impl Ins {
    /// Associate this location-independent instruction with an address.
    pub const fn at(&self, address: u32) -> LocatedIns<'_> {
        LocatedIns { instruction: self, address }
    }

    /// Return the instruction's control-flow category.
    pub const fn control_flow(&self) -> ControlFlow {
        match self {
            Self::Bt { .. } | Self::Bf { .. } => ControlFlow::Conditional { delay_slot: false },
            #[cfg(feature = "sh2")]
            Self::Bts { .. } | Self::Bfs { .. } => ControlFlow::Conditional { delay_slot: true },
            Self::Bra { .. } => ControlFlow::Direct { call: false, delay_slot: true },
            Self::Bsr { .. } => ControlFlow::Direct { call: true, delay_slot: true },
            Self::JmpAtRn { .. } => ControlFlow::Indirect { call: false, delay_slot: true },
            Self::JsrAtRn { .. } => ControlFlow::Indirect { call: true, delay_slot: true },
            #[cfg(feature = "sh2")]
            Self::BrafRn { .. } => ControlFlow::Indirect { call: false, delay_slot: true },
            #[cfg(feature = "sh2")]
            Self::BsrfRn { .. } => ControlFlow::Indirect { call: true, delay_slot: true },
            Self::Rts => ControlFlow::Return { exception: false, delay_slot: true },
            Self::Rte => ControlFlow::Return { exception: true, delay_slot: true },
            Self::Trapa { .. } => ControlFlow::Trap,
            _ => ControlFlow::Fallthrough,
        }
    }

    /// Return whether the instruction can transfer control.
    pub const fn is_branch(&self) -> bool {
        !matches!(self.control_flow(), ControlFlow::Fallthrough)
    }

    /// Return whether control transfer executes the following delay-slot instruction.
    pub const fn has_delay_slot(&self) -> bool {
        match self.control_flow() {
            ControlFlow::Conditional { delay_slot }
            | ControlFlow::Direct { delay_slot, .. }
            | ControlFlow::Indirect { delay_slot, .. }
            | ControlFlow::Return { delay_slot, .. } => delay_slot,
            ControlFlow::Fallthrough | ControlFlow::Trap => false,
        }
    }
}

impl<'a> LocatedIns<'a> {
    /// Borrow the underlying location-independent instruction.
    pub const fn instruction(self) -> &'a Ins {
        self.instruction
    }
    /// Return the instruction address.
    pub const fn address(self) -> u32 {
        self.address
    }

    /// Resolve a direct branch destination.
    pub fn branch_target(self) -> Option<Address> {
        let address = self.address;
        match self.instruction {
            Ins::Bt { disp } | Ins::Bf { disp } => Some(Address::new(
                address.wrapping_add(4).wrapping_add_signed(i32::from(disp.value()) * 2),
            )),
            Ins::Bra { disp } | Ins::Bsr { disp } => Some(Address::new(
                address.wrapping_add(4).wrapping_add_signed(i32::from(disp.value()) * 2),
            )),
            #[cfg(feature = "sh2")]
            Ins::Bts { disp } | Ins::Bfs { disp } => Some(Address::new(
                address.wrapping_add(4).wrapping_add_signed(i32::from(disp.value()) * 2),
            )),
            _ => None,
        }
    }

    /// Resolve the address referenced by a PC-relative literal load or `mova`.
    pub fn pc_relative_address(self) -> Option<Address> {
        let address = self.address;
        match self.instruction {
            Ins::MovwAtDispPcRn { disp, .. } => Some(Address::new(
                address.wrapping_add(4).wrapping_add(u32::from(disp.value()) * 2),
            )),
            Ins::MovlAtDispPcRn { disp, .. } | Ins::Mova { disp } => Some(Address::new(
                (address.wrapping_add(4) & !3).wrapping_add(u32::from(disp.value()) * 4),
            )),
            _ => None,
        }
    }
}
