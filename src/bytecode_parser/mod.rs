pub mod writer;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum TokenOpCodes {
    AddToBeginning = 0x01,
    AddToEnd = 0x02,
    DeleteFirst = 0x03,
    DeleteLast = 0x04,
    TrimBothSides = 0x05,
    TrimLeftSide = 0x06,
    TrimRightSide = 0x07,
    DeleteChunk = 0x08,
    DeleteAfter = 0x09,
    DeleteBefore = 0x0a,
    ReplaceAllWith = 0x0b,
    ReplaceFirstWith = 0x0c,
    Repeat = 0x0d,
    RotateLeft = 0x0e,
    RotateRight = 0x0f,
}
pub enum Operand {
    Str(String),
    Int(usize),
}

pub struct BytecodeInstruction {
    pub op_code: TokenOpCodes,
    pub operands: Vec<Operand>,
}

pub trait BytecodeTokenMethods {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String>;
    fn token_to_bytecode_instruction(&self) -> Result<String, String>;
}
