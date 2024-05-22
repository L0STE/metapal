use anchor_lang::prelude::*;

use mpl_core::types::{DataState, PluginAuthorityPair, OracleValidation, ExternalValidationResult};

use crate::state::{ProtocolConfig, MetapalExperience};

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct CreatePalArgs {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

#[derive(Accounts)]
#[instruction(args: CreatePalArgs)]
pub struct CreatePal<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"protocol_config", collection.key().as_ref()],
        bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    #[account(
        init,
        payer = payer,
        space = MetapalExperience::INIT_SPACE,
        seeds = [b"metapal_experience", asset.key().as_ref()],
        bump,
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

impl CreatePal<'_> {
    pub fn create_metapal_experience_account(&mut self, bump: u8) -> Result<()> {
        self.metapal_experience.set_inner(
            MetapalExperience {
                current_experience: 0,
                max_experience: self.protocol_config.starting_experience,
                validation: OracleValidation::V1 
                    { 
                        create: ExternalValidationResult::Pass, 
                        transfer: ExternalValidationResult::Pass, 
                        burn: ExternalValidationResult::Pass, 
                        update: ExternalValidationResult::Rejected,  
                    },
                protocol_config: self.protocol_config.key(),
                bump,
            }
        );

        Ok(())
    }
    pub fn create_core_asset(&self, args: CreatePalArgs) -> Result<()> {
        mpl_core::instructions::CreateV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: Some(self.collection.as_ref()),
            authority: None,
            payer: &self.payer.to_account_info(),
            owner: None,
            update_authority: None,
            system_program: &self.system_program.to_account_info(),
            log_wrapper: None,
            __program: &self.mpl_core_program.to_account_info(),
            __args: mpl_core::instructions::CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreatePal>, args: CreatePalArgs) -> Result<()> {

    ctx.accounts.create_metapal_experience_account(ctx.bumps.metapal_experience)?;
    
    ctx.accounts.create_core_asset(args)?;

    Ok(())
}

