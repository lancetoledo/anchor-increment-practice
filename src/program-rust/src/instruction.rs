use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

// Add a debug attribute to enum so we can print later
#[derive(Debug)]
// Enum can two values a sucess or failure(error)
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32)
}

 impl HelloInstruction {
     pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //  Deconstruct the first index of the array (The transaction)
        let (&tag,rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        // Figure out which Enum you want to return
        match tag {
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            // Parse the last 4 remaining indexes in the array
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => {
                        return Ok(HelloInstruction::Set(u32::from_le_bytes(i))) 
                    },
                    // If not sucessful, and is a catch all
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
        
     }
 }