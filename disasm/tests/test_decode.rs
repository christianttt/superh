use superh::{BranchTarget, Ins, Options, Reg, parse, parse_with_discriminant};

fn opts() -> Options {
    Options::default()
}

fn dis(word: u16) -> String {
    let ins = parse(word, 0, &opts());
    format!("{}", ins.display(&opts()))
}

// ─── Data Transfer ───────────────────────────────────────────────────────────

#[test]
fn mov_rm_rn() {
    // mov r2, r3  →  0110 0011 0010 0011  =  0x6323
    let ins = parse(0x6323, 0, &opts());
    assert_eq!(ins, Ins::MovRmRn { rn: Reg::R3, rm: Reg::R2 });
    assert_eq!(dis(0x6323), "mov r2, r3");
}

#[test]
fn mov_imm_rn() {
    // mov #1, r0  →  1110 0000 0000 0001  =  0xe001
    let ins = parse(0xe001, 0, &opts());
    assert_eq!(ins, Ins::MovImmRn { rn: Reg::R0, imm: 1 });
    assert_eq!(dis(0xe001), "mov #0x1, r0");
}

#[test]
fn mov_neg_imm_rn() {
    // mov #-1, r0  →  1110 0000 1111 1111  =  0xe0ff
    let ins = parse(0xe0ff, 0, &opts());
    assert_eq!(ins, Ins::MovImmRn { rn: Reg::R0, imm: -1i8 });
    assert_eq!(dis(0xe0ff), "mov #-0x1, r0");
}

#[test]
fn movb_rm_at_rn() {
    // mov.b r1, @r2  →  0010 0010 0001 0000  =  0x2210
    assert_eq!(dis(0x2210), "mov.b r1, @r2");
}

#[test]
fn movl_at_rm_rn() {
    // mov.l @r4, r5  →  0110 0101 0100 0010  =  0x6542
    let ins = parse(0x6542, 0, &opts());
    assert_eq!(ins, Ins::MovlAtRmRn { rn: Reg::R5, rm: Reg::R4 });
}

#[test]
fn movb_r0_at_disp_rn() {
    // mov.b r0, @(2, r1)  →  1000 0000 0001 0010  =  0x8012
    assert_eq!(dis(0x8012), "mov.b r0, @(0x2, r1)");
}

#[test]
fn movl_rm_at_disp_rn() {
    // mov.l r2, @(12, r3)  →  0001 0011 0010 0011  =  0x1323
    assert_eq!(dis(0x1323), "mov.l r2, @(0xc, r3)");
}

#[test]
fn movl_at_disp_rm_rn() {
    // mov.l @(4, r1), r0  →  0101 0000 0001 0001  =  0x5011
    assert_eq!(dis(0x5011), "mov.l @(0x4, r1), r0");
}

#[test]
fn movb_at_rm_inc_rn() {
    // mov.b @r0+, r1  →  0110 0001 0000 0100  =  0x6104
    assert_eq!(dis(0x6104), "mov.b @r0+, r1");
}

#[test]
fn mova() {
    // mova @(0x4, pc), r0  =  0xc700 (raw disp=0, display offset = 0*4+4-(pc&3) = 4)
    let ins = parse(0xc700, 0, &opts());
    assert_eq!(ins, Ins::Mova { disp: 4 });
}

#[test]
fn movt() {
    // movt r4  →  0000 0100 0010 1001  =  0x0429
    let ins = parse(0x0429, 0, &opts());
    assert_eq!(ins, Ins::Movt { rn: Reg::R4 });
    assert_eq!(dis(0x0429), "movt r4");
}

#[test]
fn swap_b() {
    // swap.b r1, r2  →  0110 0010 0001 1000  =  0x6218
    assert_eq!(dis(0x6218), "swap.b r1, r2");
}

// ─── PC-relative addressing ───────────────────────────────────────────────────

#[test]
fn movl_at_disp_pc_rn_aligned() {
    // mov.l @(d=1, pc), r0  at pc=0x1000 (4-byte aligned)
    // EA = 1*4 + (0x1000 & !3) + 4 = 0x1008; display_offset = 8
    let ins = parse(0xd001, 0x1000, &opts());
    assert_eq!(ins, Ins::MovlAtDispPcRn { rn: Reg::R0, disp: 8 });
    assert_eq!(format!("{}", ins.display(&opts())), "mov.l @(0x8, pc), r0");
}

#[test]
fn movl_at_disp_pc_rn_unaligned() {
    // mov.l @(d=1, pc), r0  at pc=0x1002 (not 4-byte aligned)
    // EA = 1*4 + (0x1002 & !3) + 4 = 0x1008; display_offset = 6
    let ins = parse(0xd001, 0x1002, &opts());
    assert_eq!(ins, Ins::MovlAtDispPcRn { rn: Reg::R0, disp: 6 });
    assert_eq!(format!("{}", ins.display(&opts())), "mov.l @(0x6, pc), r0");
}

#[test]
fn movw_at_disp_pc_rn_no_alignment() {
    // mov.w @(d=1, pc), r0 — no alignment correction regardless of PC
    // display_offset = d*2 + 4 = 6 always
    let ins_a = parse(0x9001, 0x0000, &opts());
    let ins_b = parse(0x9001, 0x1002, &opts());
    assert_eq!(ins_a, Ins::MovwAtDispPcRn { rn: Reg::R0, disp: 1 });
    assert_eq!(format!("{}", ins_a.display(&opts())), "mov.w @(0x6, pc), r0");
    assert_eq!(format!("{}", ins_b.display(&opts())), "mov.w @(0x6, pc), r0");
}

#[test]
fn mova_unaligned_pc() {
    // mova @(d=0, pc), r0  at pc=0x1002 (not 4-byte aligned)
    // EA = 0*4 + (0x1002 & !3) + 4 = 0x1004; display_offset = 2
    let ins = parse(0xc700, 0x1002, &opts());
    assert_eq!(ins, Ins::Mova { disp: 2 });
    assert_eq!(format!("{}", ins.display(&opts())), "mova @(0x2, pc), r0");
}

// ─── Arithmetic ──────────────────────────────────────────────────────────────

#[test]
fn add_rm_rn() {
    // add r1, r2  →  0011 0010 0001 1100  =  0x321c
    let ins = parse(0x321c, 0, &opts());
    assert_eq!(ins, Ins::AddRmRn { rn: Reg::R2, rm: Reg::R1 });
    assert_eq!(dis(0x321c), "add r1, r2");
}

#[test]
fn add_imm_rn() {
    // add #4, r0  →  0111 0000 0000 0100  =  0x7004
    assert_eq!(dis(0x7004), "add #0x4, r0");
}

#[test]
fn cmp_eq_imm() {
    // cmp/eq #0, r0  →  1000 1000 0000 0000  =  0x8800
    assert_eq!(dis(0x8800), "cmp/eq #0x0, r0");
}

#[test]
fn cmp_eq_rm_rn() {
    // cmp/eq r3, r4  →  0011 0100 0011 0000  =  0x3430
    assert_eq!(dis(0x3430), "cmp/eq r3, r4");
}

#[test]
fn neg() {
    // neg r1, r2  →  0110 0010 0001 1011  =  0x621b
    assert_eq!(dis(0x621b), "neg r1, r2");
}

#[test]
fn exts_b() {
    // exts.b r0, r1  →  0110 0001 0000 1110  =  0x610e
    assert_eq!(dis(0x610e), "exts.b r0, r1");
}

// ─── Logic ───────────────────────────────────────────────────────────────────

#[test]
fn and_rm_rn() {
    // and r2, r3  →  0010 0011 0010 1001  =  0x2329
    assert_eq!(dis(0x2329), "and r2, r3");
}

#[test]
fn or_imm_r0() {
    // or #0xff, r0  →  1100 1011 1111 1111  =  0xcbff
    assert_eq!(dis(0xcbff), "or #0xff, r0");
}

#[test]
fn xor_rm_rn() {
    // xor r5, r6  →  0010 0110 0101 1010  =  0x265a
    assert_eq!(dis(0x265a), "xor r5, r6");
}

#[test]
fn tst_imm_r0() {
    // tst #1, r0  →  1100 1000 0000 0001  =  0xc801
    assert_eq!(dis(0xc801), "tst #0x1, r0");
}

// ─── Shift / Rotate ──────────────────────────────────────────────────────────

#[test]
fn shll() {
    // shll r0  →  0100 0000 0000 0000  =  0x4000
    let ins = parse(0x4000, 0, &opts());
    assert_eq!(ins, Ins::ShllRn { rn: Reg::R0 });
    assert_eq!(dis(0x4000), "shll r0");
}

#[test]
fn shlr16() {
    // shlr16 r3  →  0100 0011 0010 1001  =  0x4329
    assert_eq!(dis(0x4329), "shlr16 r3");
}

#[test]
fn rotcl() {
    // rotcl r5  →  0100 0101 0010 0100  =  0x4524
    assert_eq!(dis(0x4524), "rotcl r5");
}

// ─── Branch ──────────────────────────────────────────────────────────────────

#[test]
fn bt() {
    // bt +8  →  1000 1001 0000 0011  =  0x8903  (disp=3 → offset=3*2+4=10)
    let ins = parse(0x8903, 0, &opts());
    assert_eq!(ins, Ins::Bt { disp: BranchTarget { addr: 10 } });
    assert_eq!(dis(0x8903), "bt 0xa");
}

#[test]
fn bf() {
    // bf .+0  →  1000 1011 1111 1110  =  0x8bfe  (disp=-2 → offset=-2*2+4=0)
    let ins = parse(0x8bfe, 0, &opts());
    assert_eq!(ins, Ins::Bf { disp: BranchTarget { addr: 0 } });
}

#[test]
fn bra() {
    // bra .+4  →  1010 0000 0000 0000  =  0xa000  (disp=0 → 0*2+4=4)
    let ins = parse(0xa000, 0, &opts());
    assert_eq!(ins, Ins::Bra { disp: BranchTarget { addr: 4 } });
    assert_eq!(dis(0xa000), "bra 0x4");
}

#[test]
fn jmp() {
    // jmp @r14  →  0100 1110 0010 1011  =  0x4e2b
    let ins = parse(0x4e2b, 0, &opts());
    assert_eq!(ins, Ins::JmpAtRn { rn: Reg::R14 });
    assert_eq!(dis(0x4e2b), "jmp @r14");
}

#[test]
fn jsr() {
    // jsr @r0  →  0100 0000 0000 1011  =  0x400b
    assert_eq!(dis(0x400b), "jsr @r0");
}

#[test]
fn rts() {
    // rts  →  0000 0000 0000 1011  =  0x000b
    let ins = parse(0x000b, 0, &opts());
    assert_eq!(ins, Ins::Rts);
    assert_eq!(dis(0x000b), "rts");
}

#[test]
fn rte() {
    // rte  →  0000 0000 0010 1011  =  0x002b
    let ins = parse(0x002b, 0, &opts());
    assert_eq!(ins, Ins::Rte);
    assert_eq!(dis(0x002b), "rte");
}

// ─── System ──────────────────────────────────────────────────────────────────

#[test]
fn nop() {
    // nop  →  0000 0000 0000 1001  =  0x0009
    let ins = parse(0x0009, 0, &opts());
    assert_eq!(ins, Ins::Nop);
    assert_eq!(dis(0x0009), "nop");
}

#[test]
fn clrt() {
    // clrt  →  0000 0000 0000 1000  =  0x0008
    let ins = parse(0x0008, 0, &opts());
    assert_eq!(ins, Ins::Clrt);
}

#[test]
fn sett() {
    // sett  →  0000 0000 0001 1000  =  0x0018
    let ins = parse(0x0018, 0, &opts());
    assert_eq!(ins, Ins::Sett);
}

#[test]
fn trapa() {
    // trapa #0x20  →  1100 0011 0010 0000  =  0xc320
    let ins = parse(0xc320, 0, &opts());
    assert_eq!(ins, Ins::Trapa { imm: 0x20 });
    assert_eq!(dis(0xc320), "trapa #0x20");
}

#[test]
fn stc_sr_rn() {
    // stc sr, r0  →  0000 0000 0000 0010  =  0x0002
    let ins = parse(0x0002, 0, &opts());
    assert_eq!(ins, Ins::StcSrRn { rn: Reg::R0 });
    assert_eq!(dis(0x0002), "stc sr, r0");
}

#[test]
fn sts_pr_rn() {
    // sts pr, r14  →  0000 1110 0010 1010  =  0x0e2a
    let ins = parse(0x0e2a, 0, &opts());
    assert_eq!(ins, Ins::StsPrRn { rn: Reg::R14 });
    assert_eq!(dis(0x0e2a), "sts pr, r14");
}

#[test]
fn lds_rm_pr() {
    // lds r14, pr  →  0100 1110 0010 1010  =  0x4e2a
    let ins = parse(0x4e2a, 0, &opts());
    assert_eq!(ins, Ins::LdsRmPr { rm: Reg::R14 });
    assert_eq!(dis(0x4e2a), "lds r14, pr");
}

#[test]
fn stsl_pr_at_dec_rn() {
    // sts.l pr, @-r15  →  0100 1111 0010 0010  =  0x4f22
    let ins = parse(0x4f22, 0, &opts());
    assert_eq!(ins, Ins::StslPrAtDecRn { rn: Reg::R15 });
    assert_eq!(dis(0x4f22), "sts.l pr, @-r15");
}

#[test]
fn ldsl_at_rm_inc_pr() {
    // lds.l @r15+, pr  →  0100 1111 0010 0110  =  0x4f26
    let ins = parse(0x4f26, 0, &opts());
    assert_eq!(ins, Ins::LdslAtRmIncPr { rm: Reg::R15 });
    assert_eq!(dis(0x4f26), "lds.l @r15+, pr");
}

// ─── Unknown encoding ────────────────────────────────────────────────────────

#[test]
fn unknown_word() {
    let ins = parse(0xffff, 0, &opts());
    assert_eq!(ins, Ins::Word(0xffff));
    assert_eq!(dis(0xffff), ".word 0xffff");
}

// ─── parse_with_discriminant ─────────────────────────────────────────────────

#[test]
fn parse_with_discriminant_roundtrip() {
    // Every recognised instruction word must round-trip: re-parsing with its
    // own discriminant must produce the same Ins value.
    let opts = opts();
    for word in 0u16..=0xffff {
        let ins = parse(word, 0, &opts);
        let disc = ins.discriminant();
        let again = parse_with_discriminant(word, disc, 0, &opts);
        assert_eq!(ins, again, "round-trip failed for word 0x{word:04x}");
    }
}

#[test]
fn parse_with_discriminant_unknown_falls_back() {
    // An out-of-range discriminant must return Ins::Word.
    let opts = opts();
    let result = parse_with_discriminant(0x6323, 0xffff, 0, &opts);
    assert_eq!(result, Ins::Word(0x6323));
}

// ─── imm_decimal option ───────────────────────────────────────────────────────

#[test]
fn imm_decimal_uimm() {
    // add #8, r3 — uimm displayed as decimal
    let opts = Options { imm_decimal: true, ..Options::default() };
    let ins = parse(0x7308, 0, &opts);
    assert_eq!(format!("{}", ins.display(&opts)), "add #8, r3");
}

#[test]
fn imm_decimal_simm() {
    // mov #100, r0
    let opts = Options { imm_decimal: true, ..Options::default() };
    let ins = parse(0xe064, 0, &opts);
    assert_eq!(format!("{}", ins.display(&opts)), "mov #100, r0");
}

#[test]
fn imm_decimal_negative() {
    // mov #-1, r0  (0xe0ff)
    let opts = Options { imm_decimal: true, ..Options::default() };
    let ins = parse(0xe0ff, 0, &opts);
    assert_eq!(format!("{}", ins.display(&opts)), "mov #-1, r0");
}

#[test]
fn imm_hex_default() {
    // Default: hex output unchanged
    let ins = parse(0xe064, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "mov #0x64, r0");
}

#[test]
fn parse_with_discriminant_specific() {
    // mov r2, r3 (0x6323) → MovRmRn; discriminant must restore it.
    let opts = opts();
    let ins = parse(0x6323, 0, &opts);
    let disc = ins.discriminant();
    let again = parse_with_discriminant(0x6323, disc, 0, &opts);
    assert_eq!(again, Ins::MovRmRn { rn: Reg::R3, rm: Reg::R2 });
}
