use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
//use std::mem; // Tried this too
//cannot use due to issues running 'npm run build'
//use solana_sdk::program::get_program_data_len;
// used for encoding data for communicating between programs, was doing research trying to get msg!() to work
//use borsh::{BorshDeserialize, BorshSerialize};
//use solana_program::program_error::ProgramError;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    _accounts: &[AccountInfo], // accounts to not interact with (this time)
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("[lib] Hello World Rust program entrypoint");
    //string interpolation not supported like with println!()
    msg!("This is the program id: {}", _program_id.to_string()); //this worked!! 
    //Logged in the 'solana logs' tab as expected

    //msg!("This is the program size: {:?}", mem::size_of::<Self>()); //Tried this, not valid in functions

    Ok(())
}
