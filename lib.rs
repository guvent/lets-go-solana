use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// import and create an instance from instruction.rs file...
pub mod instructions;
use crate::instructions::HelloInstruction;

// Define the type of state stored in accounts.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings...
    pub counter: u32,
}

// Declare and export the program's entrypoint...
entrypoint!(process_instruction);

// Program entrypoint's implementation....
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded intro...
    accounts: &[AccountInfo], // The account to say hello to ...
    instruction_data: &[u8], // Ignored, all helloworld instruction are hellos...
) -> ProgramResult {
    msg!("Hello World! Started....");

    if instruction_data.is_empty() {
        msg!("No instruction!");
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction = HelloInstruction::unpack(instruction_data)?;

    let account_iter = &mut accounts.iter();

    let account = next_account_info(account_iter)?;

    if account.owner != program_id {
        msg!("Invalid account owner!");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greetin_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        HelloInstruction::Increment => {
            greetin_account.counter += 1;
        }
        HelloInstruction::Decrement => {
            greetin_account.counter -= 1;
        }
        HelloInstruction::Reset => {
            greetin_account.counter = 0;
        }
    }

    greetin_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greetin_account.counter);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use std::mem;
    use solana_program::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let owner = Pubkey::default();
        let key = Pubkey::default();

        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let increment_instruction_data: Vec<u8> = vec![0];
        let decrement_instruction_data: Vec<u8> = vec![1];
        let reset_instruction_data: Vec<u8> = vec![2];

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
