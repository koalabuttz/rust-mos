use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Mos MosInlineAsmRegClass {
        reg,
        reg_a,
        reg_x,
        reg_y,
        reg_xy,
        reg_gpr,
    }
}

impl MosInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I8, I16; },
            Self::reg_a | Self::reg_x | Self::reg_y | Self::reg_xy | Self::reg_gpr => {
                types! { _: I8; }
            }
        }
    }
}

def_regs! {
    Mos MosInlineAsmReg MosInlineAsmRegClass {
        a: reg_a, reg_gpr = ["a"],
        x: reg_x, reg_xy, reg_gpr = ["x"],
        y: reg_y, reg_xy, reg_gpr = ["y"],

        #error = ["s", "sp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["p"] =>
            "the status register cannot be used as an operand for inline asm",
    }
}

impl MosInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
