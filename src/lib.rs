use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
    account_info::next_account_info,
    program::invoke_signed,
    sysvar::rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub mod instruction;
pub mod state;

use instruction::MovieInstruction;
use state::MovieAccountState;


entrypoint!(process_instruction);

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
}


// Instruction for adding movie-review
pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    _rating: u8,
) -> ProgramResult {
    let acccount_info_iter = &mut accounts.iter();

    // Getting Accounts
    let initializer  = next_account_info(acccount_info_iter)?;
    let pda_account  = next_account_info(acccount_info_iter)?;
    let system_program  = next_account_info(acccount_info_iter)?;

    // Deriving the PDA
    let (_movie_review_pda,bump) = Pubkey::find_program_address(
        &[initializer.key.as_ref(),title.as_bytes().as_ref()],
        program_id
    );

    let movie_review_len = 1 + 1 + (4 + title.len()) + (4 + description.len());

    //Calculating the rent
    let rent = Rent::get()?.minimum_balance(movie_review_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent,
            movie_review_len.try_into().unwrap(),
            program_id
         ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone()
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump],
        ]]

    ).unwrap();

    msg!("Unpacking the state account");

    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow())?;
    msg!("Borrowed the account");

    account_data.title = title;
    account_data.rating = _rating;
    account_data.description = description;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_program_test::*;
    #[allow(unused_imports)]
    use solana_sdk::{signature::Signer, transaction::Transaction};

    #[tokio::test]
    async fn test_hello() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("native_sol_01", program_id, processor!(process_instruction))
                .start()
                .await;
    }
}



// +++++++++++++++++++++++++ Key learnings for native-solana-programs +++++++++++++++++++++++++
// - borsh crate provides some "traits" like "BorshSerialize" and "BorshDeserialize" for serializing and deserializing the
//   Rust data(Structs or Enums) to byte array.
//
