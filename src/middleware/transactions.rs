use super::crypto::CryptoUtils;
use crate::middleware::config::ConfigIface;
use alloy::{
    network::{eip2718::Encodable2718, EthereumSigner, TransactionBuilder},
    primitives::{Address, Bytes, FixedBytes, Uint},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::eth::TransactionRequest,
    signers::wallet::LocalWallet,
    sol,
    sol_types::SolCall,
    transports::http::{Client, Http},
};
use std::str::FromStr;
use SemaphoreNetworkHSS::{
    addSubscriberAndKeyCall, getSubscriberKeyCall, SemaphoreNetworkHSSInstance,
};

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
    pub(crate) hss_address: String,
    //RPC address used for smart contract interaction.
    pub(crate) rpc: String,
    //Web3 lib HSS contract object.
    pub(crate) hss_contract: SemaphoreNetworkHSSInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl SemaphoreTransactions {
    pub fn new() -> Self {
        let hss_address_with_prefix = ConfigIface::get_config().hss_address;
        let hss_address = CryptoUtils::remove_prefix(hss_address_with_prefix);

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
            hss_address,
            rpc,
            hss_contract,
        }
    }

    /// Adds subscriber and their uncompressed public key to the HSS contract storage.
    pub async fn add_sub_and_key(
        &self,
        web3_account: LocalWallet,
        uncompressed_public_key: String,
    ) -> FixedBytes<32> {
        let provider = ProviderBuilder::new().on_http(
            self.rpc
                .parse()
                .expect("Could not read the 'rpc_url' from config.json!"),
        );

        let sender_address = web3_account.address();
        let contract_address = Address::from_str(&self.hss_address)
            .expect("Could not read the 'hss_address' from config.json!");

        let chain_id = provider
            .get_chain_id()
            .await
            .expect("Could not read the 'Chain ID' from Provider!");

        let add_sub_and_key_call = addSubscriberAndKeyCall {
            newSubscriber: sender_address,
            publicKey: uncompressed_public_key.into(),
        };
        let encoded_add_sub_and_key = add_sub_and_key_call.abi_encode();
        let encoded_bytes = Bytes::from_iter(encoded_add_sub_and_key.iter().cloned());

        let nonce = provider
            .get_transaction_count(sender_address)
            .await
            .expect("Could not read the 'Nonce' from Provider!");

        let add_pub_key_tx = TransactionRequest::default()
            .to(contract_address)
            .input(encoded_bytes.into())
            .from(sender_address)
            .with_chain_id(chain_id)
            .gas_limit(700_000)
            .max_fee_per_gas(2_000_000_000)
            .max_priority_fee_per_gas(1_000_000_000)
            .nonce(nonce);

        let signed_tx_envelope = add_pub_key_tx
            .build(&EthereumSigner::new(web3_account.clone()))
            .await
            .expect("Datas in the Tx are incorrect!");

        let tx_encoded = signed_tx_envelope.encoded_2718();

        let receipt = provider
            .send_raw_transaction(&tx_encoded)
            .await
            .expect("Datas in the Tx are incorrect!")
            .get_receipt()
            .await
            .expect("Datas in the Tx are incorrect!");

        let tx_hash = receipt.transaction_hash;

        println!("Send transaction: {}", tx_hash);

        tx_hash
    }

    /// Helper fn to query the Subscribers network ID (SNID).
    pub fn get_subscriber_pub_key(&self, subscriber_id: Uint<256, 4>) -> String {
        let get_sub_key = getSubscriberKeyCall {
            subscriberIndex: subscriber_id,
        };

        let encoded_add_sub_and_key = get_sub_key.abi_encode();
        let encoded_bytes = Bytes::from_iter(encoded_add_sub_and_key.iter());
        let encoded_str = encoded_bytes.to_string();

        encoded_str
    }
}
