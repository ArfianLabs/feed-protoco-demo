use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::mem::size_of;
declare_id!("5fiZruYuczm75PhTWhkoug5xaZUPtLEmzqc2AWYyZDzM");

#[program]
mod test_rng {
    use anchor_lang::solana_program::{instruction::Instruction, program::{get_return_data, invoke}};

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.data.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }

    pub fn get_random(ctx: Context<GetRand>) -> Result<()> {
        let rng_program = ctx.accounts.rng_program.key;
   
        
        let instruction = Instruction {
            program_id: *rng_program,
            accounts: vec![
                ctx.accounts.signer.to_account_metas(Some(true))[0].clone(),
                ctx.accounts.feed_account_1.to_account_metas(Some(false))[0].clone(),
                ctx.accounts.feed_account_2.to_account_metas(Some(false))[0].clone(),
                ctx.accounts.feed_account_3.to_account_metas(Some(false))[0].clone(),
                ctx.accounts.fallback_account.to_account_metas(Some(false))[0].clone(),
                ctx.accounts.current_feeds_account.to_account_metas(Some(false))[0].clone(),
                ctx.accounts.temp.to_account_metas(Some(true))[0].clone(),
                ctx.accounts.system_program.to_account_metas(Some(false))[0].clone(),
            ],
            data: vec![0],
        };

        let account_infos = &[
            ctx.accounts.signer.to_account_info().clone(),
            ctx.accounts.feed_account_1.to_account_info().clone(),
            ctx.accounts.feed_account_2.to_account_info().clone(),
            ctx.accounts.feed_account_3.to_account_info().clone(),
            ctx.accounts.fallback_account.to_account_info().clone(),
            ctx.accounts.current_feeds_account.to_account_info().clone(),
            ctx.accounts.temp.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ];

        invoke(&instruction, account_infos)?;

        let returned_data: (Pubkey, Vec<u8>) = get_return_data().unwrap();

        let random_number: RandomNumber;
        if &returned_data.0 == rng_program {
            let random_number = RandomNumber::try_from_slice(&returned_data.1)?;
            msg!("{}", random_number.random_number);
        } else {
            return Err(ErrorCode::FailedToGetRandomNumber.into());
        }
        Ok(())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RandomNumber {
    pub random_number: u64,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Failed to get random number.")]
    FailedToGetRandomNumber,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"data"],
        bump,
        payer = signer,
        space = size_of::<NewAccount>()*4,
    )]
    pub data: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetRand<'info> {
    #[account(mut)]
    pub data: Account<'info, NewAccount>,
    /// CHECK:
    pub feed_account_1: AccountInfo<'info>,
    /// CHECK:
    pub feed_account_2: AccountInfo<'info>,
    /// CHECK:
    pub feed_account_3: AccountInfo<'info>,
    /// CHECK:
    pub fallback_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub current_feeds_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub temp: Signer<'info>,

    /// CHECK:
    pub rng_program: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
