// use std::str::FromStr;

use solana_program::{
    account_info::{AccountInfo,next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionData {
    pub amount:u64
}
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    let json_str=std::str::from_utf8(_instruction_data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;
    let payment_details:InstructionData=serde_json::from_str(json_str).map_err(|_|solana_program::program_error::ProgramError::InvalidInstructionData)?;
    msg!("Amount :{}", payment_details.amount);
    let accounts_iter=&mut accounts.iter();
    let payer_account=next_account_info(accounts_iter)?;
    let escrow_account=next_account_info(accounts_iter)?;
    msg!("payer account :{}",payer_account.key);
    msg!("escrow account :{}",escrow_account.key);
    if **payer_account.lamports.borrow() < payment_details.amount{
        return Err(solana_program::program_error::ProgramError::InsufficientFunds);
    }
    if !escrow_account.is_writable {
        msg!("Ensure its writable");
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }
    if !payer_account.is_signer{
        msg!("Ensure its signed");
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }
    msg!("Balance of payee is {}",payer_account.lamports.borrow());
    // do remember that the amount is in lamports
     let transfer_instruction = solana_program::system_instruction::transfer(
         &payer_account.key, 
         &escrow_account.key, 
         payment_details.amount,
     );

     // Invoke the transfer instruction
     solana_program::program::invoke(
         &transfer_instruction,
         &[payer_account.clone(),escrow_account.clone()],
     )?;
     msg!("Transfer successfully completed here");

    Ok(())

}


