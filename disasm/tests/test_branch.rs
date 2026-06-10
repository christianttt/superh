use superh::{Options, parse};

fn opts() -> Options {
    Options::default()
}

// ─── Delay slot detection ────────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh1")]
fn is_delayed_branch_true() {
    assert!(parse(0xa000, 0, &opts()).is_delayed_branch()); // bra
    assert!(parse(0xb000, 0, &opts()).is_delayed_branch()); // bsr
    assert!(parse(0x402b, 0, &opts()).is_delayed_branch()); // jmp @r0
    assert!(parse(0x400b, 0, &opts()).is_delayed_branch()); // jsr @r0
    assert!(parse(0x000b, 0, &opts()).is_delayed_branch()); // rts
    assert!(parse(0x002b, 0, &opts()).is_delayed_branch()); // rte
}

#[test]
#[cfg(feature = "sh2")]
fn is_delayed_branch_sh2() {
    assert!(parse(0x0023, 0, &opts()).is_delayed_branch()); // braf r0
    assert!(parse(0x0003, 0, &opts()).is_delayed_branch()); // bsrf r0
    assert!(parse(0x8d00, 0, &opts()).is_delayed_branch()); // bt.s
    assert!(parse(0x8f00, 0, &opts()).is_delayed_branch()); // bf.s
}

#[test]
fn is_delayed_branch_false() {
    assert!(!parse(0xe001, 0, &opts()).is_delayed_branch()); // mov #1, r0
    assert!(!parse(0x8900, 0, &opts()).is_delayed_branch()); // bt (non-delayed)
    assert!(!parse(0x8b00, 0, &opts()).is_delayed_branch()); // bf (non-delayed)
    assert!(!parse(0xffff, 0, &opts()).is_delayed_branch()); // .word
}

// ─── is_branch ──────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh1")]
fn is_branch_true_delayed() {
    // All delayed branches are also branches.
    assert!(parse(0xa000, 0, &opts()).is_branch()); // bra
    assert!(parse(0xb000, 0, &opts()).is_branch()); // bsr
    assert!(parse(0x402b, 0, &opts()).is_branch()); // jmp @r0
    assert!(parse(0x400b, 0, &opts()).is_branch()); // jsr @r0
    assert!(parse(0x000b, 0, &opts()).is_branch()); // rts
    assert!(parse(0x002b, 0, &opts()).is_branch()); // rte
}

#[test]
#[cfg(feature = "sh1")]
fn is_branch_true_non_delayed() {
    // BT and BF have no delay slot but are still branches.
    assert!(parse(0x8900, 0, &opts()).is_branch()); // bt
    assert!(parse(0x8b00, 0, &opts()).is_branch()); // bf
}

#[test]
#[cfg(feature = "sh2")]
fn is_branch_true_sh2() {
    assert!(parse(0x0023, 0, &opts()).is_branch()); // braf r0
    assert!(parse(0x0003, 0, &opts()).is_branch()); // bsrf r0
    assert!(parse(0x8d00, 0, &opts()).is_branch()); // bt.s
    assert!(parse(0x8f00, 0, &opts()).is_branch()); // bf.s
}

#[test]
fn is_branch_false() {
    assert!(!parse(0xe001, 0, &opts()).is_branch()); // mov #1, r0
    assert!(!parse(0x0009, 0, &opts()).is_branch()); // nop
    assert!(!parse(0xffff, 0, &opts()).is_branch()); // .word
}

#[test]
fn trapa_is_branch_but_not_delayed() {
    // trapa transfers control to the trap handler; it has no delay slot.
    let ins = parse(0xc320, 0, &opts()); // trapa #0x20
    assert!(ins.is_branch());
    assert!(!ins.is_delayed_branch());
}

// ─── PC-relative branch display ───────────────────────────────────────────────

#[test]
fn branch_at_nonzero_pc() {
    // bra +0  at pc=0x8c010000: target = 0x8c010000 + 4 + 0*2 = 0x8c010004
    let ins = parse(0xa000, 0x8c01_0000, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "bra 0x8c010004");
    // bt +0 at pc=0x1000: target = 0x1000 + 4 = 0x1004
    assert_eq!(format!("{}", parse(0x8900, 0x1000, &opts()).display(&opts())), "bt 0x1004");
}
