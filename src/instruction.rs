use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum MovieInstruction {
    // First Instruction is Adding Movie
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
    
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String,
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        let payload = MovieReviewPayload::try_from_slice(rest)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match variant {
            0 => Ok(Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
