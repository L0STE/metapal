use anchor_lang::prelude::*;
use mpl_core::types::OracleValidation;

#[account]
pub struct ProtocolConfig {
    pub protocol_collection: Pubkey,
    pub starting_experience: u64,
    pub bump: u8,
    //... other fields
}

impl Space for ProtocolConfig {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1;
}

#[account]
pub struct MetapalExperience {
    pub current_experience: u64,
    pub max_experience: u64,
    pub validation: OracleValidation,
    pub protocol_config: Pubkey,
    pub bump: u8,
}

impl Space for MetapalExperience {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 5;
}
