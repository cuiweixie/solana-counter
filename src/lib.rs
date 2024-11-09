use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter_data = account.try_borrow_mut_data()?;
    let counter = counter_data[0];

    match instruction_data[0] {
        0 => {
            counter_data[0] = counter.wrapping_add(1);
            msg!("Counter incremented to: {}", counter_data[0]);
        }
        1 => {
            let increment_by = instruction_data[1];
            counter_data[0] = counter.wrapping_add(increment_by);
            msg!("Counter incremented by {} to: {}", increment_by, counter_data[0]);
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}