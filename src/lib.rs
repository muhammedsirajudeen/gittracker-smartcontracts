// // use std::str::FromStr;

// use solana_program::{
//     account_info::{AccountInfo,next_account_info},
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
// };
// use serde::{Deserialize, Serialize};


// #[derive(Serialize, Deserialize, Debug)]
// pub struct InstructionData {
//     pub payee_address:String,
//     pub amount:u64
// }
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     _program_id: &Pubkey,
//     _accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let json_str=std::str::from_utf8(_instruction_data)
//         .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;
//     let payment_details:InstructionData=serde_json::from_str(json_str).map_err(|_|solana_program::program_error::ProgramError::InvalidInstructionData)?;
//     msg!("Amount :{}", payment_details.amount);
//     msg!("Payee address :{}",payment_details.payee_address);
//     let accounts_iter=&mut _accounts.iter();
//     let payer_account=next_account_info(accounts_iter)?;
//     let escrow_account=next_account_info(accounts_iter)?;
//     let system_account=next_account_info(accounts_iter)?;
//     // let escrow_account_string: Pubkey=Pubkey::from_str("5TiC68nb5fMqUwXimQK8R7MVnWxRTvtNAyDoJNpZgHh3").map_err(|_| solana_program::program_error::ProgramError::InvalidAccountData)?;
//     msg!("payer account :{}",payer_account.key);
//     msg!("escrow account :{}",escrow_account.key);
//     if **payer_account.lamports.borrow() < payment_details.amount{
//         return Err(solana_program::program_error::ProgramError::InsufficientFunds);
//     }
//     if !escrow_account.is_writable {
//         msg!("Ensure its writable");
//         return Err(solana_program::program_error::ProgramError::InvalidAccountData);
//     }
//     if !payer_account.is_signer{
//         msg!("Ensure its signed");
//         return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
//     }
//     msg!("Balance of payee is {}",payer_account.lamports.borrow());
//     msg!("System account is {}",system_account.key);
//     // do remember that the amount is in lamports
//     let transfer_instruction=solana_program::system_instruction::transfer(&escrow_account.key, &payer_account.key, payment_details.amount);
//     //the line that i have marked is failing the next line
//     let accounts=vec![
//         payer_account.clone(),
//         escrow_account.clone(),
//         system_account.clone()
//     ];
//     let transfer_result = solana_program::program::invoke(
//         &transfer_instruction,
//         &accounts,
//     );
    
//     match transfer_result {
//         Ok(_) => {
//             msg!("Transfer succeeded!");
//         }
//         Err(e) => {
//             msg!("Transfer failed with error: {:?}", e);
//             return Err(e); // Propagate the error
//         }
//     }
//     Ok(())
// }

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    msg
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // The first account is the payer (invoker)
    let payer_account = next_account_info(accounts_iter)?;
    
    // The second account is the recipient
    let recipient_account = next_account_info(accounts_iter)?;
    
    // Ensure payer account is signing the transaction
    if !payer_account.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }
    
    // Amount to transfer (in lamports)
    let transfer_amount = 1000000000; // Example: 1 SOL = 1,000,000 lamports
    
    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &payer_account.key, 
        &recipient_account.key, 
        transfer_amount,
    );
    
    // Invoke the transfer instruction
    solana_program::program::invoke(
        &transfer_instruction,
        &[payer_account.clone(), recipient_account.clone()],
    )?;
    msg!("Transfer completed");
    Ok(())
}
