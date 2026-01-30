use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod helios {
    use super::*;

    pub fn register_agent(ctx: Context<RegisterAgent>, name: String, version: String) -> Result<()> {
        let agent_account = &mut ctx.accounts.agent_account;
        agent_account.authority = ctx.accounts.authority.key();
        agent_account.name = name;
        agent_account.version = version;
        agent_account.is_active = true;
        agent_account.created_at = Clock::get()?.unix_timestamp;

        msg!("Agent Registered: {} (v{})", agent_account.name, agent_account.version);
        Ok(())
    }

    pub fn update_status(ctx: Context<UpdateStatus>, is_active: bool) -> Result<()> {
        let agent_account = &mut ctx.accounts.agent_account;
        agent_account.is_active = is_active;
        msg!("Agent Status Updated: {}", is_active);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, version: String)]
pub struct RegisterAgent<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + (4 + 64) + (4 + 16) + 1 + 8, // Discriminator + Pubkey + Name + Version + Bool + Timestamp
        seeds = [b"agent", authority.key().as_ref()],
        bump
    )]
    pub agent_account: Account<'info, AgentEntry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStatus<'info> {
    #[account(
        mut,
        seeds = [b"agent", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub agent_account: Account<'info, AgentEntry>,

    pub authority: Signer<'info>,
}

#[account]
pub struct AgentEntry {
    pub authority: Pubkey,
    pub name: String,
    pub version: String,
    pub is_active: bool,
    pub created_at: i64,
}
