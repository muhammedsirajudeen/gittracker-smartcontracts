use std::str::FromStr;

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
    pub payee_address:String,
    pub amount:u64
}
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    let json_str=std::str::from_utf8(_instruction_data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;
    let payment_details:InstructionData=serde_json::from_str(json_str).map_err(|_|solana_program::program_error::ProgramError::InvalidInstructionData)?;
    msg!("Amount :{}", payment_details.amount);
    msg!("Payee address :{}",payment_details.payee_address);
    let accounts_iter=&mut _accounts.iter();
    let payee_account=next_account_info(accounts_iter)?;
    let escrow_account=Pubkey::from_str("5TiC68nb5fMqUwXimQK8R7MVnWxRTvtNAyDoJNpZgHh3").map_err(|_| solana_program::program_error::ProgramError::InvalidAccountData)?;
    msg!("payer account :{}",payee_account.key);
    msg!("escrow account :{}",escrow_account);
    if **payee_account.lamports.borrow() < payment_details.amount{
        return Err(solana_program::program_error::ProgramError::InsufficientFunds);
    }
    let transfer_instruction=solana_program::system_instruction::transfer(&payee_account.key, &escrow_account, payment_details.amount);
    let accounts=vec![
        payee_account.clone(),
        next_account_info(accounts_iter)?.clone(),
    ];
    solana_program::program::invoke(&transfer_instruction, &accounts,)?;

    Ok(())
}