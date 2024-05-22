use anchor_lang::prelude::*;
declare_id!("H6Vt7jBb23jFL3EoKNfkAo7SQrxoJ5CxbXp3T3KS8Dw");

pub mod state;
pub mod errors;
pub mod instructions;
pub use instructions::*;

#[program]
pub mod metapal {
    use super::*;

    pub fn create_protocol_config(ctx: Context<CreateProtocolConfig>, args: CreateProtocolConfigArgs) -> Result<()> {
        instructions::create_protocol_config::handler(ctx, args)
    }

    pub fn create_pal(ctx: Context<CreatePal>, args: CreatePalArgs) -> Result<()> {
        instructions::create_pal::handler(ctx, args)
    }

    pub fn change_experience(ctx: Context<ChangeExperience>, args: ChangeExperienceArgs) -> Result<()> {
        instructions::change_experience::handler(ctx, args)
    }

    pub fn level_up(ctx: Context<LevelUp>, args: LevelUpArgs) -> Result<()> {
        instructions::level_up::handler(ctx, args)
    }
}