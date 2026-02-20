use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::commit, ephem::commit_accounts};
use ephemeral_vrf_sdk::{consts::VRF_PROGRAM_IDENTITY, rnd::random_u8_with_range};

use crate::state::UserAccount;

#[commit]
#[derive(Accounts)]
pub struct CallbackRandomUpdateCommit<'info> {
    #[account(mut, address = VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> CallbackRandomUpdateCommit<'info> {
    pub fn callback_random_update_commit(&mut self, randomness: [u8; 32]) -> Result<()> {
        self.user_account.data = random_u8_with_range(&randomness, 1, 100) as u64;

        commit_accounts(
            &self.vrf_program_identity.to_account_info(),
            vec![&self.user_account.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}
