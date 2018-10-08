use self::*;

pub type c_long = i32;
pub type c_ulong = u32;
pub type c_char = u8;

pub const PT_STEP: ::c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
