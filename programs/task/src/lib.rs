use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use num_traits::pow;

use std::mem::size_of;
declare_id!("8xhpMe7GWpSSxwpWEUF6YJMr6hkgVEPWSkWkZcjQ35ZK");

#[program]
pub mod farming {
    pub const RINO_MINT_ADDRESS: &str = "GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY";
    use super::*;
    pub fn create_lp_token_bag(
        ctx: Context<CreateLpTokenBag>,
        lp_mint_address: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
    pub fn intialize_user_profile(
        ctx: Context<InitializeUserProfile>,
        lp_mint_address: Pubkey,
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_farming_profile;
        user_profile.staked_accounts = 0;
        user_profile.total_staked_amount = 0.0;
        user_profile.mint = lp_mint_address;
        Ok(())
    }
    pub fn stake_lp(
        ctx: Context<StakeLp>,
        program_fino_bag_bump: u8,
        index: String,
        lp_mint: Pubkey,
        user_pda: Pubkey,
        user_profile_bump: u8,
        amount: f64,
    ) -> Result<()> {
        let lpStakeTranscation: &mut Account<LpStakeTransaction> =
            &mut ctx.accounts.lp_stake_transaction;
        let decimals = ctx.accounts.lp_mint_.decimals;
        let stake_profile = &mut ctx.accounts.user_profile;
        lpStakeTranscation.amount = 0.0;
        lpStakeTranscation.owner = user_pda;
        let amount_decimals: f64 = amount * pow(10.0, decimals as usize);
        let cpi_ctx: CpiContext<token::Transfer> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_lp_token_bag.to_account_info(),
                authority: ctx.accounts.user_lp_token_bag_authority.to_account_info(),
                to: ctx.accounts.program_lp_token_bag.to_account_info(),
            },
        );
        msg!("The amount is {}", decimals as usize);
        msg!("Amount in decimal is {}", amount_decimals);

        token::transfer(cpi_ctx, amount_decimals as u64)?;
        stake_profile.staked_accounts = stake_profile.staked_accounts + 1;
        stake_profile.total_staked_amount = stake_profile.total_staked_amount + amount;
        lpStakeTranscation.amount = lpStakeTranscation.amount + amount;
        lpStakeTranscation.seed = index.parse::<u64>().unwrap();
        lpStakeTranscation.mint = lp_mint;
        lpStakeTranscation.last_harvest = 0;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        lpStakeTranscation.start_time = current_timestamp;

        Ok(())
    }

    pub fn harvest(
        ctx: Context<Harvest>,
        rino_mint_authority_bump: u8,
        lp_mint: Pubkey,
        user_profile_bump: u8,
        lp_stake_transaction_bump: u8,
        index: String,
    ) -> Result<()> {
        let decimals = ctx.accounts.rino_mint.decimals;
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        let lpStakeTranscation = &mut ctx.accounts.lp_stake_transaction;
        if lpStakeTranscation.amount <= 0.0 {
            return err!(ErrorCode::InsufficentBalance);
        }

        let mut time = 0;
        if lpStakeTranscation.last_harvest > 0 {
            time = clock.unix_timestamp - lpStakeTranscation.last_harvest;
        } else {
            time = clock.unix_timestamp - lpStakeTranscation.start_time;
        }
        lpStakeTranscation.last_harvest = current_time;
        let min: f64 = (time as f64) / 60.0;
        if min <= 1.0 {
            return err!(ErrorCode::InsufficentRewardBalance);
        }

        let rino_mint_address = ctx.accounts.rino_mint.key();
        let seeds = &[rino_mint_address.as_ref(), &[rino_mint_authority_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.rino_mint.to_account_info(),
                to: ctx.accounts.user_rino_token_bag.to_account_info(),
                authority: ctx.accounts.rino_mint_authority.to_account_info(),
            },
            &signer,
        );
        msg!("min is {}", min);
        msg!("pool start time  is {}", lpStakeTranscation.start_time);
        msg!("current time is   is {}", current_time);
        let amount_decimals = lpStakeTranscation.amount * pow(10.0, decimals as usize);
        token::mint_to(cpi_ctx, (amount_decimals * min * 1000.0) as u64)?;

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        program_lp_bag_bump: u8,
        rino_mint_authority_bump: u8,
        user_profile_bump: u8,
        lp_stake_transaction_bump: u8,
        lp_mint: Pubkey,
        index: String,
    ) -> Result<()> {
        let decimals = ctx.accounts.lp_mint_.decimals;
        let lpStakeTranscation: &mut Account<LpStakeTransaction> =
            &mut ctx.accounts.lp_stake_transaction;
        let stake_profile = &mut ctx.accounts.user_profile;
        stake_profile.total_staked_amount =
            stake_profile.total_staked_amount - lpStakeTranscation.amount;
        let clock = Clock::get()?;
        let stakedtime = clock.unix_timestamp - lpStakeTranscation.start_time;
        if stakedtime < 120 {
            return err!(ErrorCode::TimeLocked);
        }
        if lpStakeTranscation.amount > 0.0 {
            let lp_mint_address = ctx.accounts.lp_mint_.key();
            let seeds = &[lp_mint_address.as_ref(), &[program_lp_bag_bump]];
            let signer = [&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.program_lp_token_bag.to_account_info(),
                    authority: ctx.accounts.program_lp_token_bag.to_account_info(),
                    to: ctx.accounts.user_lp_token_bag.to_account_info(),
                },
                &signer,
            );
            msg!("Pool |AMoutnt is {}", lpStakeTranscation.amount);

            let amount_decimals: f64 = lpStakeTranscation.amount * pow(10.0, decimals as usize);
            lpStakeTranscation.amount = 0.0;
            token::transfer(cpi_ctx, amount_decimals as u64)?;
        } else {
            return err!(ErrorCode::InsufficentBalance);
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey)]
pub struct InitializeUserProfile<'info> {
    //   PDA
    #[account(
        init,
        seeds = [b"FarmingProfilee".as_ref(),lp_mint.key().as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = size_of::<UserLpStakingProfile>() + 16
    )]
    pub user_farming_profile: Account<'info, UserLpStakingProfile>,
    #[account(address= lp_mint_address)]
    pub lp_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LpStakeTransaction {
    owner: Pubkey,
    amount: f64,
    start_time: i64,
    seed: u64,
    last_harvest: i64,
    mint: Pubkey,
}
#[account]
pub struct UserLpStakingProfile {
    total_staked_amount: f64,
    staked_accounts: u32,
    mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey)]
pub struct CreateLpTokenBag<'info> {
    // PDA
    #[account(
        init,
        payer = payer,
        seeds = [ lp_mint.key().as_ref() ],
        bump,
        token::mint = lp_mint,
        // PDA authority!
        token::authority = program_lp_token_bag,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,

    #[account(
        address = lp_mint_address,
    )]
    pub lp_mint: Account<'info, Mint>,

    // rent
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(program_lp_bag_bump: u8,index: String,lp_mint:Pubkey,user_pda:Pubkey,user_profile_bump:u8)]
pub struct StakeLp<'info> {
    #[account(mut,seeds=[b"FarmingProfilee".as_ref(),lp_mint.key().as_ref(),user.key().as_ref()],bump=user_profile_bump)]
    pub user_profile: Account<'info, UserLpStakingProfile>,

    #[account(
        init,
        seeds = [b"LpStakeTransaction".as_ref(),user_profile.key().as_ref(),index.as_ref()],
        bump,
        payer = user,
        space = size_of::<LpStakeTransaction>() + 16
    )]
    pub lp_stake_transaction: Account<'info, LpStakeTransaction>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_lp_token_bag: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_token_bag_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ lp_mint.key().as_ref() ],
        bump = program_lp_bag_bump,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,

    #[account(address= lp_mint)]
    pub lp_mint_: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction( rino_mint_authority_bump:u8,lp_mint:Pubkey,user_profile_bump:u8,lp_stake_transaction_bump:u8,index:String)]
pub struct Harvest<'info> {
    #[account(mut,seeds=[b"FarmingProfilee".as_ref(),lp_mint_.key().as_ref(),user.key().as_ref()],bump=user_profile_bump)]
    pub user_profile: Account<'info, UserLpStakingProfile>,

    #[account(mut,seeds=[b"LpStakeTransaction".as_ref(),user_profile.key().as_ref(),index.as_ref()],bump=lp_stake_transaction_bump)]
    pub lp_stake_transaction: Account<'info, LpStakeTransaction>,

    pub token_program: Program<'info, Token>,

    // minting
    #[account(
        mut,
        address = RINO_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
        )]
    pub rino_mint: Account<'info, Mint>,
    #[account(
        seeds = [ rino_mint.key().as_ref() ],
        bump = rino_mint_authority_bump,
        )]
    /// CHECK: LATER
    pub rino_mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address= lp_mint)]
    pub lp_mint_: Account<'info, Mint>,

    #[account(mut)]
    pub user_rino_token_bag: Account<'info, TokenAccount>,
}
#[derive(Accounts)]
#[instruction(program_lp_bag_bump: u8,rino_mint_authority_bump:u8,user_profile_bump:u8,lp_stake_transaction_bump:u8,lp_mint:Pubkey,index:String)]
pub struct Withdraw<'info> {
    #[account(mut,seeds=[b"FarmingProfilee".as_ref(),lp_mint_.key().as_ref(),user.key().as_ref()],bump=user_profile_bump)]
    pub user_profile: Account<'info, UserLpStakingProfile>,

    #[account(mut,seeds=[b"LpStakeTransaction".as_ref(),user_profile.key().as_ref(),index.as_ref()],bump=lp_stake_transaction_bump)]
    pub lp_stake_transaction: Account<'info, LpStakeTransaction>,

    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        seeds = [lp_mint.as_ref()],
        bump = program_lp_bag_bump,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_lp_token_bag: Account<'info, TokenAccount>,

    #[account(address= lp_mint)]
    pub lp_mint_: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficent Fino Balance")]
    InsufficentBalance,
    #[msg("Insufficent Reward Balance")]
    InsufficentRewardBalance,
    #[msg("Time Locked")]
    TimeLocked,
}
