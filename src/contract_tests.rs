#[cfg(test)]
mod tests {
    use alloy::signers::wallet::LocalWallet;

    use crate::middleware::{
        config::ConfigIface,
        transactions::{self, SemaphoreTransactions},
    };

    #[test]
    fn contract_test() {
        // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
        // The following code is for testing only. Set up signer from private key, be aware of danger.
        let semaphore = SemaphoreTransactions::new();
        let priv_key_from_file = ConfigIface::get_config().private_key;
        let signer: LocalWallet = priv_key_from_file
            .parse()
            .expect("The Private Key was given in config.json file has invalid form!");
        let uncompressed = "c6b754b20826eb925e052ee2c25285b162b51fdca732bcf67e39d647fb6830aeb651944a574a362082a77e3f2b5d9223eb54d7f2f76846522bf75f3bedb8178e";

        let _ = transactions::add_sub_and_key(semaphore, signer, uncompressed.to_string());
    }
}
