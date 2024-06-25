use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey,
    system_instruction::transfer, transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

#[collection]
pub mod my_actions {
    use super::*;

    pub fn send_donation(
        ctx: ContextWithQuery<SendDonationAction, SendDonationQuery>,
    ) -> Result<Transaction> {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::from(ActionError::InvalidAccountPublicKey)),
        };
        let receiver_pubkey = pubkey!("6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G");
        let transfer_instruction = transfer(
            &account_pubkey,
            &receiver_pubkey,
            ctx.query.amount * LAMPORTS_PER_SOL,
        );
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}

#[derive(Action)]
#[action(
    icon = "https://<icon-url>",
    title = "Alice's website",
    description = "Website to make a donation to Alice",
    label = "Send",
    link = {
        label = "Send 1 SOL",
        href = "https://<api-url>?amount=1",
    },
    link = {
        label = "Send 5 SOL",
        href = "https://<api-url>?amount=5",
    },
    link = {
        label = "Custom Donation",
        href = "https://<api-url>?amount={amount}",
        parameter = { label = "Amount in SOL", name = "amount" }
    },
)]
pub struct SendDonationAction;

#[query]
pub struct SendDonationQuery {
    pub amount: u64,
}

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
}