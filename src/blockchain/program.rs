use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Receipt {
    pub id: String,
    pub items: Vec<Item>,
    pub total: f64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Add receipt processing logic here
    Ok(())
}