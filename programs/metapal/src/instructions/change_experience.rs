use anchor_lang::prelude::*;
use mpl_core::types::{OracleValidation, ExternalValidationResult};

use crate::{
    state::{ProtocolConfig, MetapalExperience},
    errors::MetapalError,
};

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct ChangeExperienceArgs {
    pub experience_amount: u64,
    pub add: bool,
}

#[derive(Accounts)]
#[instruction(args: ChangeExperienceArgs)]
pub struct ChangeExperience<'info> {
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

    pub system_program: Program<'info, System>,
}

impl ChangeExperience<'_> {
    pub fn modify_experience(&mut self, args: ChangeExperienceArgs) -> Result<()> {
        let mut experience = self.metapal_experience.clone();

        if args.add {
            if args.experience_amount >= experience.max_experience.checked_sub(experience.current_experience).ok_or(MetapalError::Overflow)? {
                experience.current_experience = experience.max_experience;
                experience.validation = OracleValidation::V1 { 
                    create: ExternalValidationResult::Pass, 
                    transfer: ExternalValidationResult::Pass, 
                    burn: ExternalValidationResult::Pass, 
                    update: ExternalValidationResult::Approved, 
                };
            } else {
                experience.current_experience = experience.current_experience.checked_add(args.experience_amount).ok_or(MetapalError::Overflow)?;
            }
        } else {
            require!(experience.current_experience >= args.experience_amount, MetapalError::Underflow);
            experience.current_experience = experience.current_experience.checked_sub(args.experience_amount).ok_or(MetapalError::Underflow)?;
        }

        Ok(())
    }
}

pub fn handler(ctx: Context<ChangeExperience>, args: ChangeExperienceArgs) -> Result<()> {
    
    ctx.accounts.modify_experience(args)?;
    
    Ok(())
}

