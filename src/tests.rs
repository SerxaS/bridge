#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::middleware::{
        config::ConfigIface,
        crypto::{CryptoUtils, PubKey},
    };
    use alloy::{
        contract::{ContractInstance, Interface},
        json_abi::JsonAbi,
        primitives::{keccak256, Address},
    };
    use secp256k1::{PublicKey, SecretKey};

    // Answers
    // TODO: Should be 04 at the beginning of correct uncompressed pub key!!
    const CORRECT_UNCOMPRESSED_PUBKEY: &str = "c6b754b20826eb925e052ee2c25285b162b51fdca732bcf67e39d647fb6830aeb651944a574a362082a77e3f2b5d9223eb54d7f2f76846522bf75f3bedb8178e";
    const CORRECT_HASHED_UC_PUBKEY: &str =
        "0xe8b0087eec10090b15f4fc4bc96aaa54e2d44c299564da76e1cd3184a2386b8d";
    const CORRECT_ETHEREUM_ADDRESS: &str = "0xc96aaa54e2d44c299564da76e1cd3184a2386b8d";

    #[test]
    fn test_get_uncompressed_pub_key() {
        // TODO: Different from Python!!!!!!!!!! I created new abi file! Should i add comment to them?
        // TODO: What is "a" variable = true?
        // TODO: There is lamda function at decompress pubkey func why?

        let adress_from_json = ConfigIface::get_config().hss_address;
        let address = Address::parse_checksummed(adress_from_json, None)
            .expect("The length of the address was given incorrect!");

        let provider = ConfigIface::get_config().rpc_url;

        // Open config file and parse JSON.
        let abi_path = "src/middleware/artifacts/Abi.json";
        let abi_str = std::fs::read_to_string(abi_path)
            .expect("Could not open the file. Please provide a SemaphoreNetworkHSS.json in `artifacts` directory.");
        let abi_data: JsonAbi = serde_json::from_str(&abi_str)
            .expect("JSON was not well-formatted! Unable to read the data!");

        let interface = Interface::new(abi_data.clone());

        let contract: ContractInstance<Address, String, Interface> =
            ContractInstance::new(address, provider, interface);

        // TODO: Changed name of variables. Different from python. Check!!
        let priv_key_from_file = ConfigIface::get_config().private_key;
        let priv_key_without_prefix = CryptoUtils::remove_prefix(priv_key_from_file);
        let priv_key = SecretKey::from_str(priv_key_without_prefix.as_str())
            .expect("Could not read the Private Key from config.json!");

        let curve = CryptoUtils::get_curve();

        let pub_key = PublicKey::from_secret_key(&curve, &priv_key);
        let uncomp_pub_key_byte_04 = pub_key.serialize_uncompressed();
        let uncomp_pub_key_04 = hex::encode(uncomp_pub_key_byte_04);
        let uncomp_pub_key = CryptoUtils::remove_prefix(uncomp_pub_key_04);

        let mut uncomp_pub_key_byte = uncomp_pub_key_byte_04.to_vec();
        uncomp_pub_key_byte.remove(0);

        let hashed_uncomp_pub_key_byte = keccak256(&uncomp_pub_key_byte);
        let hashed_uncomp_pub_key_without_prefix = hex::encode(hashed_uncomp_pub_key_byte);
        let hashed_uncomp_pub_key = CryptoUtils::add_prefix(&hashed_uncomp_pub_key_without_prefix);

        // TODO: show how got 26(Written on Python. Meaning?)
        let eth_adress_char: Vec<_> = hashed_uncomp_pub_key_without_prefix.chars().collect();
        let mut eth_adress_temp = vec!['0'; 40];

        for i in 24..64 {
            eth_adress_temp[i - 24] = eth_adress_char[i];
        }

        let eth_adress_without_prefix: String = eth_adress_temp.into_iter().collect();
        let eth_adress = CryptoUtils::add_prefix(&eth_adress_without_prefix);

        assert_eq!(CORRECT_UNCOMPRESSED_PUBKEY, uncomp_pub_key);
        assert_eq!(CORRECT_HASHED_UC_PUBKEY, hashed_uncomp_pub_key);
        assert_eq!(CORRECT_ETHEREUM_ADDRESS, eth_adress);
    }

    #[test]
    fn test_decompress_pub_key() {
        let correct_uncomp_pub_key_vec: Vec<_> = CORRECT_UNCOMPRESSED_PUBKEY.chars().collect();

        let mut x_coord_temp = vec!['0'; 64];
        let mut y_coord_temp = vec!['0'; 64];

        for i in 0..64 {
            x_coord_temp[i] = correct_uncomp_pub_key_vec[i]
        }

        for i in 64..128 {
            y_coord_temp[i - 64] = correct_uncomp_pub_key_vec[i]
        }

        let x_coord: String = x_coord_temp.into_iter().collect();
        let x_coord_prefix = CryptoUtils::add_prefix(&x_coord);

        let y_coord: String = y_coord_temp.into_iter().collect();
        let y_coord_prefix = CryptoUtils::add_prefix(&y_coord);

        println!("Public Key's x coordinate: {:?}", x_coord_prefix);
        println!("Public Key's y coordinate: {:?}", y_coord_prefix);

        let pub_key = PubKey::new(x_coord, y_coord);

        let comp_pub_key = CryptoUtils::compress(pub_key);
        println!("Compressed Public Key: {:?}", comp_pub_key);
        let decomp_pub_key = CryptoUtils::decompress_public_key(comp_pub_key);
        // TODO added 04 prefix. If not need delete!!
        println!("Decompressed Public Key: {:?}", decomp_pub_key);
    }
}
