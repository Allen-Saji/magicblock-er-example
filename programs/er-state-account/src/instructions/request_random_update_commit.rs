use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::{
    anchor::vrf,
    consts::DEFAULT_QUEUE,
    instructions::{create_request_randomness_ix, RequestRandomnessParams},
    types::SerializableAccountMeta,
};

#[vrf]
#[derive(Accounts)]
pub struct RequestRandomUpdateCommit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: PDA verified by seeds; account may be delegated to ER so we can't deserialize it
    #[account(
        seeds = [b"user", payer.key().as_ref()],
        bump,
    )]
    pub user_account: AccountInfo<'info>,
    /// CHECK: Oracle queue account validated by address constraint
    #[account(mut, address = DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

impl<'info> RequestRandomUpdateCommit<'info> {
    pub fn request_random_update_commit(&self, client_seed: u8) -> Result<()> {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.payer.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator:
                crate::instruction::CallbackRandomUpdateCommit::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.user_account.key(),
                is_signer: false,
                is_writable: true,
            }]),
            callback_args: None,
        });

        self.invoke_signed_vrf(&self.payer.to_account_info(), &ix)?;

        Ok(())
    }
}
