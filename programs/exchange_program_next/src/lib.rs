#[macro_export]
macro_rules! solana_exchange_program_next {
    () => {
        (
            "solana_exchange_program_next".to_string(),
            solana_exchange_api_next::id(),
        )
    };
}
use solana_exchange_api_next::exchange_processor::process_instruction;

solana_sdk::solana_entrypoint!(process_instruction);
