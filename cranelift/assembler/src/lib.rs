#![allow(non_camel_case_types)]

mod alloc;
pub mod fuzz;
mod imm;
mod mem;
mod reg;
mod rex;
mod sink;

use alloc::RegallocVisitor;
use imm::{Imm16, Imm32, Imm8};
use mem::{emit_modrm_sib_disp, GprMem};
use reg::{Gpr, Size};
use rex::{emit_simm, RexFlags};
use sink::CodeSink;

// Include code generated by the `meta` crate; this
include!(concat!(env!("OUT_DIR"), "/assembler.rs"));

/// Helper function to make code generation simpler.
fn emit_modrm(buffer: &mut impl CodeSink, enc_reg_g: u8, rm_e: u8) {
    let modrm = rex::encode_modrm(0b11, enc_reg_g & 7, rm_e & 7);
    buffer.put1(modrm);
}
