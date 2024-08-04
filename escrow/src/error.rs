use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<EscrowError> for ProgramError {
    fn from(value: EscrowError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
