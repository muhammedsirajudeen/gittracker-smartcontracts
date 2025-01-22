use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    system_instruction,
};



entrypoint!(process_instruction);

pub fn process_instruction(    
	program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],) -> ProgramResult {
	
	msg!("Program Successfully Executed");
	Ok(())
}