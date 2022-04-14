pub mod amm_instruction;
pub mod amm_stats;
pub mod error;
pub mod state;
#[cfg(test)]
pub mod tests;
#[cfg(not(target ="bpf"))]
pub mod routes;
#[cfg(not(target ="bpf"))]
pub mod rpc;

solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");