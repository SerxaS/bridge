use crate::middleware::config::ConfigIface;
use alloy::{
    contract::{ContractInstance, Interface},
    json_abi::JsonAbi,
    primitives::Address,
};

/// Struct for on-chain interactions with deployed contracts.
/// A valid web3 account with signer must be passed into contract interaction functions.
/// Solely intended for use by SemaphoreAccount in account.
struct SemaphoreTransactions {
    //HSS smart contract ABI.
    hss_abi: JsonAbi,
    //HSS smart contract address.
    hss_adress: String,
    //Web3 lib HSS contract object.
    hss_contract: ContractInstance<Address, String, Interface>,
    //RPC address used for smart contract interaction.
    rpc: String,
}

impl SemaphoreTransactions {
    pub fn transaction() -> Self {
        let rpc = ConfigIface::get_config().rpc_url;

        let hss_adress = ConfigIface::get_config().hss_address;

        let adress_from_json = ConfigIface::get_config().hss_address;
        let address = Address::parse_checksummed(adress_from_json, None)
            .expect("The length of the address was given incorrect!");

        let provider = ConfigIface::get_config().rpc_url;

        // TODO: Check abi data inside semaphore!!!
        // Open config file and parse JSON.
        let hss_abi_path = "src/middleware/artifacts/Abi.json";
        let hss_abi_str = std::fs::read_to_string(hss_abi_path)
            .expect("Could not open the file. Please provide a Abi.json in `artifacts` directory.");
        let hss_abi: JsonAbi = serde_json::from_str(&hss_abi_str)
            .expect("JSON was not well-formatted! Unable to read the data!");

        let interface = Interface::new(hss_abi.clone());

        let hss_contract: ContractInstance<Address, String, Interface> =
            ContractInstance::new(address, provider, interface);

        Self {
            hss_abi,
            hss_adress,
            hss_contract,
            rpc,
        }
    }

    // /// Adds subscriber and their uncompressed public key to the HSS contract storage.
    // fn add_sub_and_key(web3_account: String, uncompressed_pub_key: String) -> String {

    // }
}
