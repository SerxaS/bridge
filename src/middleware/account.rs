use super::{config::ConfigIface, transactions::SemaphoreTransactions};
use crate::middleware::crypto::*;
use alloy::hex::ToHexExt;
use alloy::primitives::Uint;
use alloy::signers::wallet::LocalWallet;
use num_bigint::BigUint;
use num_traits::Num;

/// TODO: Check prints in get_shared_secret!
/// Wrapper for managing a given web3 account.
pub struct SemaphoreNetworkAccount {
    pub(crate) crypto: CryptoUtils,
    pub(crate) web3_account: Option<Box<LocalWallet>>,
    pub(crate) transactions: SemaphoreTransactions,
}
impl SemaphoreNetworkAccount {
    /// Generates Web3 Account if necessary.
    fn new(config: ConfigIface, web3_account: Option<Box<LocalWallet>>) -> Self {
        if web3_account.is_none() {
            let signer: LocalWallet = config
                .private_key
                .parse()
                .expect("The Private Key was given in config.json file has invalid form!");
            // Creates a signer from Private Key. Note that the strings cannot be prefixed with 0x.
            let web3_account: Option<Box<LocalWallet>> = Some(Box::new(signer));
        }

        let curve = CryptoUtils::get_curve();
        let crypto = CryptoUtils { curve };
        let transactions = SemaphoreTransactions::new();

        Self {
            crypto,
            web3_account,
            transactions,
        }
    }

    /// Returns the nested web3 account object.
    fn get_web3_account(&self) -> Box<LocalWallet> {
        self.web3_account
            .clone()
            .expect("The Account has invalid form!")
    }

    /// Given a subscriber index, get a shared secret key between subscriber and this provider.
    fn get_shared_secret(&self, subscriber_index: Uint<256, 4>) -> BigUint {
        println!("Subscriber SUID index to lookup on chain (RAW) {subscriber_index}");

        // Query web3 rpc @subscriber index, cast to int.
        let subscriber_pub = self.transactions.get_subscriber_pub_key(subscriber_index);
        println!("Subscriber @{subscriber_index}'s (compressed) pubkey is {subscriber_pub}");

        let subscriber_pub_hex = subscriber_pub.encode_hex();
        println!("Subscribers network ID (SNID): {subscriber_pub_hex}");

        // Generate shared secret.
        let shared_secret = self.crypto.gen_shared_secret(
            self.web3_account
                .clone()
                .expect("The Account has invalid form!"),
            subscriber_pub,
        );

        shared_secret
    }
}
