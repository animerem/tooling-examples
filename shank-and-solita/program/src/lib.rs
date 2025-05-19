mod instructions;
mod state;

use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo,
        declare_id,
        entrypoint,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        msg,
    },
};

use crate::instructions::*;

/// Program ID
declare_id!("8avNGHVXDwsELJaWMSoUZ44CirQd4zyU9Ez4ZmP4jNjZ");

/// Entrypoint for the Solana program
entrypoint!(process_instruction);

/// Processes incoming instructions dispatched by Solana runtime
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize instruction data
    let instruction = match CarRentalServiceInstruction::try_from_slice(instruction_data) {
        Ok(ix) => ix,
        Err(e) => {
            msg!("Failed to deserialize instruction data: {:?}", e);
            return Err(e.into());
        }
    };

    // Match instruction variant and call respective handler
    match instruction {
        CarRentalServiceInstruction::AddCar(car) => {
            msg!("Instruction: AddCar");
            add_car(program_id, accounts, car)
        },
        CarRentalServiceInstruction::BookRental(order) => {
            msg!("Instruction: BookRental");
            book_rental(program_id, accounts, order)
        },
        CarRentalServiceInstruction::PickUpCar => {
            msg!("Instruction: PickUpCar");
            pick_up_car(program_id, accounts)
        },
        CarRentalServiceInstruction::ReturnCar => {
            msg!("Instruction: ReturnCar");
            return_car(program_id, accounts)
        },
    }
}
