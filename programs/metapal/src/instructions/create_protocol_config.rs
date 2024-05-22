use anchor_lang::prelude::*;

use mpl_core::types::PluginAuthorityPair;

use crate::state::ProtocolConfig;

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct CreateProtocolConfigArgs {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

#[derive(Accounts)]
#[instruction(args: CreateProtocolConfigArgs)]
pub struct CreateProtocolConfig<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = ProtocolConfig::INIT_SPACE,
        seeds = [b"protocol_config", collection.key().as_ref()],
        bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    /// CHECK: mpl-core program will check this for us
    pub collection: UncheckedAccount<'info>,

    /// CHECK: mpl-core program will check this for us
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl CreateProtocolConfig<'_> {
    pub fn create_protocol_config_account(&mut self, bump: u8) -> Result<()> {
        
        self.protocol_config.set_inner(
            ProtocolConfig {
                protocol_collection: self.collection.key(),
                starting_experience: 100,
                bump,
            }
        );

        Ok(())
    }

    pub fn create_core_collection(&self, args: CreateProtocolConfigArgs) -> Result<()> {
        
        mpl_core::instructions::CreateCollectionV1Cpi {
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            update_authority: Some(&self.protocol_config.as_ref()),
            system_program: &self.system_program.to_account_info(),
            __program: &self.mpl_core_program.to_account_info(),
            __args: mpl_core::instructions::CreateCollectionV1InstructionArgs {
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateProtocolConfig>, args: CreateProtocolConfigArgs) -> Result<()> {
    
    ctx.accounts.create_protocol_config_account(ctx.bumps.protocol_config)?;

    ctx.accounts.create_core_collection(args)?;

    Ok(())
}

