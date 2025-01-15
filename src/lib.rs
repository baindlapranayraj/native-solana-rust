pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// +++++++++++++++++++++++++ Key learnings for native-solana-programs +++++++++++++++++++++++++
// - borsh crate provides some "traits" like "BorshSerialize" and "BorshDeserialize" for serializing and deserializing the
//   Rust data(Structs or Enums) to byte array.
// - PDAs are often considered to be trusted stores of a program's state.
//   Ensuring the correct program owns the PDAs is a fundamental way to prevent malicious behavior.
// - Think like an attacker if u want to secure your program.