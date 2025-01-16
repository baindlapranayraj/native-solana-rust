# Movie Review Solana Program

This Solana program allows users to add and update movie reviews on the blockchain using PDAs (Program Derived Addresses) for account management.

## Features

- Add a movie review with a title, rating (0-5), and description.
- Update an existing movie review.

## Instructions

### Add Movie Review

1. Derive a PDA using the user's public key and the movie title.
2. Validate:
   - Rating should be between 0 and 5.
   - Total data length should not exceed 1000 bytes.
3. Create the account with rent-exempt balance.
4. Save the review details (title, rating, description) to the account.

### Update Movie Review

1. Validate ownership of the PDA and check the user's signature.
2. Verify the account is already initialized.
3. Validate:
   - Rating should be between 0 and 5.
   - Total data length should not exceed 1000 bytes.
4. Update the review details in the PDA account.

## Transaction Example

Check the example transaction on Solana Explorer:  
[View Transaction](https://explorer.solana.com/tx/C6b8txVtMoG93qMh1BA34mF7VipZtcmjDVh3wWJSK8o1M4HPPLYDt7XeZj2PHoiA3tWMd6AEGwBsAZNSAeKtL2f?cluster=devnet)

## Dependencies

- Solana Program Crate
- Borsh for serialization/deserialization

## Usage

1. Clone the repository and navigate to the program directory.
2. Build the program using the Solana CLI:
   ```bash
   cargo build-bpf
