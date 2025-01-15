use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewErrors {
    #[error("Account is not initialized yet")]
    UninitializedAccount,

    #[error("PDA Derived is not equal to PDA submited")]
    InvalidPDA,

    #[error("This Account Owner is not belongs to you")]
    InvalidAccountOwner,

    #[error("Movie Review number is not valid")]
    InvalidMovieReview,

    #[error("Input data exceeds max length")]
    InvalidDataLength,
}

// Converting our error to Solana Program Custoum Error
impl From<ReviewErrors> for ProgramError {
    fn from(value: ReviewErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
