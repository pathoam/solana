//! Bitcoin SPV proof verifier program
//! Receive merkle proofs and block headers, validate transaction

use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::InstructionError;


pub struct SpvProcessor {}

impl SpvProcessor {



}
pub fn process_instruction(
    _program_id: &Pubkey,
    keyed_accounts: &mut [KeyedAccount],
    data: &[u8],
) -> Result<(), InstructionError> {
    solana_logger::setup();

    let command = bincode::deserialize::<SpvInstruction>(data).map_err(|err| {
        info!("invalid transaction data: {:?} {:?}", data, err);
        InstructionError::InvalidInstructionData
    })?;

    trace!("{:?}", command);

    match command{
        SpvInstruction::VerificationRequest() => {
            SpvProcessor::do_verification_request()
        }
    }


}
