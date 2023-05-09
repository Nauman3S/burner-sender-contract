use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::burn,
    state::{Account, Mint},
};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account_info = next_account_info(accounts_iter)?;
    let user_wallet_info = next_account_info(accounts_iter)?;

    match instruction_data[0] {
        1 => {
            msg!("Burn token supply");
            let burn_amount = u64::from_le_bytes(instruction_data[1..].try_into().unwrap());
            let mint_info = next_account_info(accounts_iter)?;
            let token_account_info = next_account_info(accounts_iter)?;

            let mint = Mint::unpack_from_slice(&mint_info.data.borrow())?;
            let token_account = Account::unpack_from_slice(&token_account_info.data.borrow())?;

            if &mint.mint_authority.unwrap() != account_info.key {
                msg!("Invalid mint authority");
                return Err(ProgramError::InvalidAccountData);
            }

            if token_account_info.owner != program_id {
                msg!("Invalid token account owner");
                return Err(ProgramError::InvalidAccountData);
            }

            let burn_instruction = burn(
                &spl_token::id(),
                token_account_info.key,
                &mint_info.key,
                account_info.key,
                &[],
                burn_amount,
            )?;

            solana_program::program::invoke(&burn_instruction, &[
                token_account_info.clone(),
                mint_info.clone(),
                account_info.clone(),
            ])?;
        }
        2 => {
            msg!("Send NFT to user wallet");
            let nft_info = next_account_info(accounts_iter)?;

            if *user_wallet_info.key != *nft_info.owner {
                msg!("User wallet does not own the NFT");
                return Err(ProgramError::InvalidAccountData);
            }

            **nft_info.lamports.borrow_mut() = 1;
            **user_wallet_info.lamports.borrow_mut() += 1;
        }
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
