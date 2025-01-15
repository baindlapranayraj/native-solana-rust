use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::ReviewErrors, instruction::MovieInstruction, state::MovieAccountState};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, description, rating),
    }

    Ok(())
}

// Instruction for adding movie-review
pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    rating: u8,
) -> ProgramResult {
    let acccount_info_iter = &mut accounts.iter();

    // Getting Accounts
    let initializer = next_account_info(acccount_info_iter)?;
    let pda_account = next_account_info(acccount_info_iter)?;
    let system_program = next_account_info(acccount_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required Sign");
        return  Err(ProgramError::MissingRequiredSignature);
    }

    // Deriving the PDA
    let (movie_review_pda, bump) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );

    if movie_review_pda != *pda_account.key {
        msg!("The input PDA is not same as derived PDA");
        return Err(ReviewErrors::InvalidPDA.into())
    }

    if rating < 0 || rating > 5  {
        msg!("Invalid Rating Number");
        return Err(ReviewErrors::InvalidMovieReview);
    }

    let movie_review_len = 1 + 1 + (4 + title.len()) + (4 + description.len());

    if movie_review_len > 1000 {
        msg!("The lengthe of movie review is too high");
        return  Err(ReviewErrors::InvalidDataLength);
    }


    //Calculating the rent
    let rent = Rent::get()?.minimum_balance(movie_review_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent,
            1000,
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump]]],
    )
    .unwrap();

    msg!("Unpacking the state account");

    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow())?;
    if account_data.is_initialized {
        msg!("The Account is already created bro!");
        return  Err(ProgramError::AccountAlreadyInitialized);
    }


    msg!("Borrowed the account");

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}
