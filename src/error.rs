use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MathError {
    #[error("AddOverflow")]
    AddOverflow,
    #[error("Underflow")]
    SubUnderflow,
    #[error("MulOverflow")]
    MulOverflow,
    #[error("DividedByZero")]
    DividedByZero,
    #[error("UnableToRoundU64")]
    UnableToRoundU64,
    #[error("UnableToRoundU128")]
    UnableToRoundU128,
}

impl From<MathError> for ProgramError {
    fn from(e: MathError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MathError {
    fn type_of() -> &'static str {
        "Math Error"
    }
}
