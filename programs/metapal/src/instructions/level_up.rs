use anchor_lang::prelude::*;

use mpl_core::types::{OracleValidation, ExternalValidationResult, Plugin};

use crate::{
    state::{ProtocolConfig, MetapalExperience},
    errors::MetapalError,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LevelUpArgs {
    pub update_plugin: Plugin,
}


#[derive(Accounts)]
#[instruction(args: LevelUpArgs)]
pub struct LevelUp<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"protocol_config", protocol_config.protocol_collection.key().as_ref()],
        bump = protocol_config.bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    #[account(
        mut,
        seeds = [b"metapal_experience", asset.key().as_ref()],
        bump = metapal_experience.bump,
        has_one = protocol_config,
    )]
    pub metapal_experience: Account<'info, MetapalExperience>,

    #[account(mut)]
    /// CHECK: mpl-core program will check this for us
    pub asset: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: mpl-core program will check this for us
    pub collection: UncheckedAccount<'info>,

    /// CHECK: mpl-core program will check this for us
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl LevelUp<'_> {
    pub fn update_plugin(&mut self, args: LevelUpArgs) -> Result<()> {
        
        mpl_core::instructions::UpdatePluginV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: Some(&self.collection.to_account_info()),
            authority: Some(&self.protocol_config.to_account_info()),
            payer: &self.payer.to_account_info(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: None,
            __program: &self.mpl_core_program.to_account_info(),
            __args: mpl_core::instructions::UpdatePluginV1InstructionArgs {
                plugin: args.update_plugin,
            },
        }
        .invoke()?;

        Ok(())
    }

    pub fn update_experience_account(&mut self) -> Result<()> {
        
        let mut experience = self.metapal_experience.clone();
        experience.current_experience = 0;
        experience.max_experience = experience.max_experience.checked_add(experience.max_experience.checked_mul(10).ok_or(MetapalError::Overflow)?.checked_div(100).ok_or(MetapalError::Underflow)?).ok_or(MetapalError::Overflow)?;
        experience.validation = OracleValidation::V1 { 
            create: ExternalValidationResult::Pass, 
            transfer: ExternalValidationResult::Pass, 
            burn: ExternalValidationResult::Pass, 
            update: ExternalValidationResult::Rejected, 
        };
        
        Ok(())
    }
}

pub fn handler(ctx: Context<LevelUp>, args: LevelUpArgs) -> Result<()> {
    
    ctx.accounts.update_plugin(args)?;

    ctx.accounts.update_experience_account()?;
    
    Ok(())
}

