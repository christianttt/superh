//! Allocation-free instruction effects and architectural resources.

#[cfg(feature = "sh3")]
use crate::BankReg;
use crate::{Architecture, Reg};
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};

const MAX_RESOURCES: usize = 16;
const MAX_MEMORY_ACCESSES: usize = 3;

/// A control or system register tracked by data-flow analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SystemReg {
    /// Status register.
    Sr,
    /// Global base register.
    Gbr,
    /// Vector base register.
    Vbr,
    /// Saved status register.
    Ssr,
    /// Saved program counter.
    Spc,
    /// Saved general register.
    Sgr,
    /// Debug base register.
    Dbr,
    /// Procedure register.
    Pr,
    /// Multiply-accumulate high register.
    Mach,
    /// Multiply-accumulate low register.
    Macl,
    /// Floating-point communication register.
    Fpul,
    /// Floating-point status/control register.
    Fpscr,
    /// TRAPA exception register holding the trap immediate.
    Tra,
    /// Exception event register.
    Expevt,
}

/// An individually addressable status-register bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StatusBit {
    /// Condition bit.
    T,
    /// Multiply-accumulate saturation bit.
    S,
    /// Division quotient state bit.
    Q,
    /// Division divisor-sign state bit.
    M,
}

/// A physical SH-4 floating-point resource.
#[cfg(feature = "sh4")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FpuResource {
    /// FR-bank lane selected when FPSCR.FR is clear.
    Fr(FReg),
    /// XF-bank lane selected when FPSCR.FR is set.
    Xf(FReg),
    /// Double-precision FR-bank view.
    Dr(DReg),
    /// Double-precision XF-bank view.
    Xd(DReg),
    /// Four-lane FR-bank vector view.
    Vector(VecReg),
    /// Four-lane XF-bank vector view.
    XVector(VecReg),
    /// XMTRX matrix view over the XF bank.
    Matrix,
}

/// A register or architectural resource used by an instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Resource {
    /// General-purpose register.
    Gp(Reg),
    #[cfg(feature = "sh3")]
    /// Banked general-purpose register.
    Bank(BankReg),
    /// Whole system register.
    System(SystemReg),
    /// Individual status bit.
    Status(StatusBit),
    #[cfg(feature = "sh4")]
    /// Floating-point physical or aggregate resource.
    Fpu(FpuResource),
}

/// An allocation-free set of unique architectural resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceSet {
    resources: [Option<Resource>; MAX_RESOURCES],
    len: u8,
}

impl ResourceSet {
    pub(crate) const fn new() -> Self {
        Self { resources: [None; MAX_RESOURCES], len: 0 }
    }

    pub(crate) fn insert(&mut self, resource: Resource) {
        if self.contains(resource) {
            return;
        }
        let len = usize::from(self.len);
        assert!(len < MAX_RESOURCES, "generated resource capacity exceeded");
        self.resources[len] = Some(resource);
        self.len += 1;
    }

    /// Return whether this set contains `resource`.
    pub fn contains(&self, resource: Resource) -> bool {
        self.iter().any(|candidate| candidate == resource)
    }

    /// Iterate over resources in deterministic insertion order.
    pub fn iter(&self) -> impl Iterator<Item = Resource> + '_ {
        self.resources[..usize::from(self.len)].iter().filter_map(|resource| *resource)
    }

    /// Return the number of resources.
    pub const fn len(&self) -> usize {
        self.len as usize
    }
    /// Return whether no resources are present.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Known or unknown SH-4 FPSCR mode bits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct FpscrState {
    /// Precision mode (`PR`), or unknown.
    pub pr: Option<bool>,
    /// Transfer size mode (`SZ`), or unknown.
    pub sz: Option<bool>,
    /// Register-bank mode (`FR`), or unknown.
    pub fr: Option<bool>,
}

impl FpscrState {
    /// Construct an explicitly known or unknown FPSCR mode state.
    pub const fn new(pr: Option<bool>, sz: Option<bool>, fr: Option<bool>) -> Self {
        Self { pr, sz, fr }
    }
}

/// Machine context required to interpret instruction effects.
#[must_use = "effect contexts must be passed to instruction effect analysis"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct EffectContext {
    /// Runtime architecture whose semantics apply.
    pub architecture: Architecture,
    /// Known and unknown SH-4 mode state.
    pub fpscr: FpscrState,
}

impl EffectContext {
    /// Construct a context with unknown FPSCR mode bits.
    pub const fn new(architecture: Architecture) -> Self {
        Self { architecture, fpscr: FpscrState { pr: None, sz: None, fr: None } }
    }
    /// Attach known or unknown FPSCR mode bits.
    #[must_use = "builder methods return the updated effect context"]
    pub const fn with_fpscr(mut self, fpscr: FpscrState) -> Self {
        self.fpscr = fpscr;
        self
    }
}

impl Default for EffectContext {
    fn default() -> Self {
        Self::new(Architecture::default())
    }
}

/// Instruction control-flow behavior without a resolved destination.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlFlow {
    /// Continues to the next instruction.
    #[default]
    Fallthrough,
    /// Conditional direct transfer.
    Conditional {
        /// Whether the next instruction executes before transfer.
        delay_slot: bool,
    },
    /// Direct branch or call.
    Direct {
        /// Whether the transfer writes a return address.
        call: bool,
        /// Whether the next instruction executes before transfer.
        delay_slot: bool,
    },
    /// Register-computed branch or call.
    Indirect {
        /// Whether the transfer writes a return address.
        call: bool,
        /// Whether the next instruction executes before transfer.
        delay_slot: bool,
    },
    /// Normal or exception return.
    Return {
        /// Whether this restores exception state.
        exception: bool,
        /// Whether the next instruction executes before transfer.
        delay_slot: bool,
    },
    /// Synchronous software trap.
    Trap,
}

/// Direction of a memory access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemoryAccessKind {
    /// Read only.
    Read,
    /// Write only.
    Write,
    /// Atomic or combined read-modify-write.
    ReadWrite,
}

/// Width of a memory access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccessWidth {
    /// Eight bits.
    Byte,
    /// Sixteen bits.
    Word,
    /// Thirty-two bits.
    Long,
    /// Sixty-four bits.
    Quad,
    /// Width selected by FPSCR.SZ.
    FpscrSz,
}

/// Abstract addressing mode of a memory access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AddressingMode {
    /// Register indirect.
    Indirect,
    /// Pre-decrement register indirect.
    PreDecrement,
    /// Post-increment register indirect.
    PostIncrement,
    /// Base register plus displacement.
    Displacement,
    /// PC-relative.
    PcRelative,
    /// GBR-relative.
    Gbr,
    /// R0-indexed.
    Indexed,
}

/// One architectural memory access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoryAccess {
    /// Access direction.
    pub kind: MemoryAccessKind,
    /// Access width.
    pub width: AccessWidth,
    /// Address calculation family.
    pub addressing: AddressingMode,
}

/// Complete instruction effects. `must_*` is always a subset of `may_*`.
#[must_use = "instruction effects must be inspected to be useful"]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effects {
    must_read: ResourceSet,
    may_read: ResourceSet,
    must_write: ResourceSet,
    may_write: ResourceSet,
    memory: [Option<MemoryAccess>; MAX_MEMORY_ACCESSES],
    memory_len: u8,
    control_flow: ControlFlow,
}

impl Effects {
    /// Resources read in every interpretation allowed by the context.
    pub const fn must_read(&self) -> &ResourceSet {
        &self.must_read
    }
    /// Resources that may be read in at least one allowed interpretation.
    pub const fn may_read(&self) -> &ResourceSet {
        &self.may_read
    }
    /// Resources written in every interpretation allowed by the context.
    pub const fn must_write(&self) -> &ResourceSet {
        &self.must_write
    }
    /// Resources that may be written in at least one allowed interpretation.
    pub const fn may_write(&self) -> &ResourceSet {
        &self.may_write
    }
    /// Memory accesses in architectural order.
    pub fn memory(&self) -> impl Iterator<Item = MemoryAccess> + '_ {
        self.memory[..usize::from(self.memory_len)].iter().filter_map(|access| *access)
    }
    /// Control-flow classification.
    pub const fn control_flow(&self) -> ControlFlow {
        self.control_flow
    }
}

pub(crate) struct EffectsBuilder {
    effects: Effects,
    #[cfg_attr(not(feature = "sh4"), allow(dead_code))]
    context: EffectContext,
}

impl EffectsBuilder {
    pub(crate) const fn new(context: EffectContext, control_flow: ControlFlow) -> Self {
        Self {
            context,
            effects: Effects {
                must_read: ResourceSet::new(),
                may_read: ResourceSet::new(),
                must_write: ResourceSet::new(),
                may_write: ResourceSet::new(),
                memory: [None; MAX_MEMORY_ACCESSES],
                memory_len: 0,
                control_flow,
            },
        }
    }

    fn insert_read(&mut self, resource: Resource, definite: bool) {
        self.effects.may_read.insert(resource);
        if definite {
            self.effects.must_read.insert(resource);
        }
    }
    fn insert_write(&mut self, resource: Resource, definite: bool) {
        self.effects.may_write.insert(resource);
        if definite {
            self.effects.must_write.insert(resource);
        }
    }
    pub(crate) fn read(&mut self, resource: Resource) {
        self.insert_read(resource, true);
    }
    pub(crate) fn write(&mut self, resource: Resource) {
        self.insert_write(resource, true);
    }

    #[cfg(feature = "sh4")]
    pub(crate) fn read_freg(&mut self, reg: FReg) {
        self.freg(reg, false, true);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn write_freg(&mut self, reg: FReg) {
        self.freg(reg, true, true);
    }
    #[cfg(feature = "sh4")]
    fn fpu(&mut self, resource: FpuResource, write: bool, definite: bool) {
        if write {
            self.insert_write(Resource::Fpu(resource), definite);
        } else {
            self.insert_read(Resource::Fpu(resource), definite);
        }
    }
    #[cfg(feature = "sh4")]
    fn freg(&mut self, reg: FReg, write: bool, definite: bool) {
        let fr = self.context.fpscr.fr;
        match fr {
            Some(false) => self.fpu(FpuResource::Fr(reg), write, definite),
            Some(true) => self.fpu(FpuResource::Xf(reg), write, definite),
            None => {
                self.fpu(FpuResource::Fr(reg), write, false);
                self.fpu(FpuResource::Xf(reg), write, false);
            }
        }
    }

    #[cfg(feature = "sh4")]
    fn dreg(&mut self, reg: DReg, write: bool, definite: bool) {
        match self.context.fpscr.fr {
            Some(false) => self.fpu(FpuResource::Dr(reg), write, definite),
            Some(true) => self.fpu(FpuResource::Xd(reg), write, definite),
            None => {
                self.fpu(FpuResource::Dr(reg), write, false);
                self.fpu(FpuResource::Xd(reg), write, false);
            }
        }
    }

    #[cfg(feature = "sh4")]
    fn xdreg(&mut self, reg: DReg, write: bool, definite: bool) {
        match self.context.fpscr.fr {
            Some(false) => self.fpu(FpuResource::Xd(reg), write, definite),
            Some(true) => self.fpu(FpuResource::Dr(reg), write, definite),
            None => {
                self.fpu(FpuResource::Dr(reg), write, false);
                self.fpu(FpuResource::Xd(reg), write, false);
            }
        }
    }

    #[cfg(feature = "sh4")]
    fn precision_reg(&mut self, reg: FReg, write: bool) {
        match self.context.fpscr.pr {
            Some(false) => self.freg(reg, write, true),
            Some(true) => self.dreg(DReg::from_u8(reg.number() / 2), write, true),
            None => {
                self.freg(reg, write, false);
                self.dreg(DReg::from_u8(reg.number() / 2), write, false);
            }
        }
    }

    #[cfg(feature = "sh4")]
    fn transfer_reg(&mut self, reg: FReg, write: bool) {
        match self.context.fpscr.sz {
            Some(false) => self.freg(reg, write, true),
            Some(true) => self.transfer_double_reg(reg, write, true),
            None => {
                self.freg(reg, write, false);
                self.transfer_double_reg(reg, write, false);
            }
        }
    }

    #[cfg(feature = "sh4")]
    fn transfer_double_reg(&mut self, reg: FReg, write: bool, definite: bool) {
        let double_reg = DReg::from_u8(reg.number() / 2);
        if reg.number() & 1 == 0 {
            self.dreg(double_reg, write, definite);
        } else {
            self.xdreg(double_reg, write, definite);
        }
    }

    #[cfg(feature = "sh4")]
    pub(crate) fn read_precision_reg(&mut self, reg: FReg) {
        self.precision_reg(reg, false);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn write_precision_reg(&mut self, reg: FReg) {
        self.precision_reg(reg, true);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn read_transfer_reg(&mut self, reg: FReg) {
        self.transfer_reg(reg, false);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn write_transfer_reg(&mut self, reg: FReg) {
        self.transfer_reg(reg, true);
    }

    #[cfg(feature = "sh4")]
    pub(crate) fn read_dreg(&mut self, reg: DReg) {
        self.dreg(reg, false, true);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn write_dreg(&mut self, reg: DReg) {
        self.dreg(reg, true, true);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn read_vec(&mut self, reg: VecReg) {
        self.vec(reg, false);
    }
    #[cfg(feature = "sh4")]
    pub(crate) fn write_vec(&mut self, reg: VecReg) {
        self.vec(reg, true);
    }
    #[cfg(feature = "sh4")]
    fn vec(&mut self, reg: VecReg, write: bool) {
        match self.context.fpscr.fr {
            Some(false) => self.fpu(FpuResource::Vector(reg), write, true),
            Some(true) => self.fpu(FpuResource::XVector(reg), write, true),
            None => {
                self.fpu(FpuResource::Vector(reg), write, false);
                self.fpu(FpuResource::XVector(reg), write, false);
            }
        }
    }

    pub(crate) fn memory(&mut self, access: MemoryAccess) {
        let memory_len = usize::from(self.effects.memory_len);
        assert!(memory_len < MAX_MEMORY_ACCESSES, "generated memory capacity exceeded");
        let access = match (access.width, self.context.fpscr.sz) {
            (AccessWidth::FpscrSz, Some(false)) => {
                MemoryAccess { width: AccessWidth::Long, ..access }
            }
            (AccessWidth::FpscrSz, Some(true)) => {
                MemoryAccess { width: AccessWidth::Quad, ..access }
            }
            _ => access,
        };
        self.effects.memory[memory_len] = Some(access);
        self.effects.memory_len += 1;
    }
    pub(crate) fn finish(self) -> Effects {
        self.effects
    }
}
