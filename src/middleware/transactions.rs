use crate::middleware::config::ConfigIface;
use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    signers::wallet::LocalWallet,
    sol,
    transports::http::{Client, Http},
};
use SemaphoreNetworkHSS::SemaphoreNetworkHSSInstance;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SemaphoreNetworkHSS,
    "src/middleware/artifacts/SemaphoreNetworkHSS.json"
);

/// Struct for on-chain interactions with deployed contracts.
/// A valid web3 account with signer must be passed into contract interaction functions.
/// Solely intended for use by SemaphoreAccount in account.
pub struct SemaphoreTransactions {
    //HSS smart contract address.
    pub(crate) hss_adress: String,
    //RPC address used for smart contract interaction.
    pub(crate) rpc: String,
    //Web3 lib HSS contract object.
    pub(crate) hss_contract: SemaphoreNetworkHSSInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl SemaphoreTransactions {
    pub fn new() -> Self {
        let hss_adress = ConfigIface::get_config().hss_address;

        let rpc = ConfigIface::get_config().rpc_url;

        // Creates a contract instance.
        let provider = ProviderBuilder::new().on_http(
            rpc.parse()
                .expect("Could not read the 'rpc_url' from config.json!"),
        );
        let hss_contract = SemaphoreNetworkHSS::new(
            ConfigIface::get_config()
                .hss_address
                .parse()
                .expect("Could not read the 'hss_address' from config.json!"),
            provider,
        );

        Self {
            hss_adress,
            rpc,
            hss_contract,
        }
    }
}

/// Adds subscriber and their uncompressed public key to the HSS contract storage.
pub async fn add_sub_and_key(
    semaphore_transactions: SemaphoreTransactions,
    signer: LocalWallet,
    uncompressed_public_key: String,
) {
    // Creates a provider.
    let provider = ProviderBuilder::new().on_http(
        semaphore_transactions
            .rpc
            .parse()
            .expect("Could not read the 'rpc_url' from config.json!"),
    );

    // Gets nonce value from last TX.
    let nonce = provider
        .get_transaction_count(signer.address())
        .await
        .expect("Could not get the 'nonce' from block!");

    let add_pub_key_tx = semaphore_transactions
        .hss_contract
        .addSubscriberAndKey(signer.address(), uncompressed_public_key.into())
        .chain_id(11155111)
        .gas(700_000)
        .max_fee_per_gas(2_000_000_000)
        .max_priority_fee_per_gas(1_000_000_000)
        .nonce(nonce);

    // Build the transaction using the `EthereumSigner` with the provided signer.
    let tx_envelope = add_pub_key_tx.send().await;
}
