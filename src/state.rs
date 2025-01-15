use borsh::{BorshDeserialize,BorshSerialize};

// Defining the Sates of the given program hear
#[derive(Default,BorshDeserialize,BorshSerialize)]
pub struct MovieAccountState {
    pub is_initialized:bool,
    pub rating:u8,
    pub title:String,
    pub description:String
}
