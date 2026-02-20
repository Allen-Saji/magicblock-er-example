use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::{consts::VRF_PROGRAM_IDENTITY, rnd::random_u8_with_range};

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct CallbackRandomUpdate<'info> {
    #[account(address = VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> CallbackRandomUpdate<'info> {
    pub fn callback_random_update(&mut self, randomness: [u8; 32]) -> Result<()> {
        self.user_account.data = random_u8_with_range(&randomness, 1, 100) as u64;
        Ok(())
    }
}
