// Should be in xmas-elf see: https://github.com/nrc/xmas-elf/issues/54
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum RelocationTypes {
    R_AARCH64_ABS64,
    R_AARCH64_COPY,
    R_AARCH64_GLOB_DAT,
    R_AARCH64_JUMP_SLOT,
    R_AARCH64_RELATIVE,
    /// Unknown
    Unknown(u32),
}

impl RelocationTypes {
    // Construct a new x86_64::RelocationTypes
    pub fn from(typ: u32) -> RelocationTypes {
        use RelocationTypes::*;
        match typ {
            257 => R_AARCH64_ABS64,
            1024 => R_AARCH64_COPY,
            1025 => R_AARCH64_GLOB_DAT,
            1026 => R_AARCH64_JUMP_SLOT,
            1027 => R_AARCH64_RELATIVE,
            x => Unknown(x),
        }
    }
}
