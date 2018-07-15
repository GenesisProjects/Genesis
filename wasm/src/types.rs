#[derive(Copy, Clone, Debug)]
pub enum WASMType {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

impl WASMType {
    pub fn size_of(wasm_type: WASMType) -> usize {
        match wasm_type {
            WASMType::U8(_) => 8usize,
            WASMType::I8(_) => 8usize,
            WASMType::U16(_) => 16usize,
            WASMType::I16(_) => 16usize,
            WASMType::U32(_) => 32usize,
            WASMType::I32(_) => 32usize,
            WASMType::U64(_) => 64usize,
            WASMType::I64(_) => 64usize,
            WASMType::F32(_) => 32usize,
            WASMType::F64(_) => 64usize
        }
    }
}

impl Into<u8> for WASMType {
    fn into(self) -> u8 {
        match self {
            WASMType::U8(v) => v as u8,
            WASMType::I8(v) => v as u8,
            WASMType::U16(v) => v as u8,
            WASMType::I16(v) => v as u8,
            WASMType::U32(v) => v as u8,
            WASMType::I32(v) => v as u8,
            WASMType::U64(v) => v as u8,
            WASMType::I64(v) => v as u8,
            WASMType::F32(v) => v as u8,
            WASMType::F64(v) => v as u8,
        }
    }
}
