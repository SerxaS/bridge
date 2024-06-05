use hex;
use num_bigint::{BigInt, BigUint};
use num_traits::Num;
use prefix_hex::ToHexPrefixed;
use secp256k1::{ecdh::SharedSecret, All, PublicKey, Secp256k1, SecretKey};
use std::str::FromStr;

pub struct PubKey {
    pub(crate) x_coord: String,
    pub(crate) y_coord: String,
}

impl PubKey {
    pub fn new(x_coord: String, y_coord: String) -> Self {
        Self { x_coord, y_coord }
    }
}

/// secp256k1 curve.
pub struct CryptoUtils {
    curve: Secp256k1<All>,
}

impl CryptoUtils {
    pub fn get_curve() -> Secp256k1<All> {
        let curve = Secp256k1::new();

        curve
    }

    /// Helper function for compresses the Public Key's x & y coordinates into common format.
    pub fn compress(pub_key: PubKey) -> String {
        // Byte representation of x & y coordinates.
        let mut pub_key_x_byte = BigUint::from_str_radix(&pub_key.x_coord, 16)
            .unwrap()
            .to_bytes_le()
            .to_vec();

        let pub_key_y_byte = BigUint::from_str_radix(&pub_key.y_coord, 16)
            .unwrap()
            .to_bytes_le();

        // If Public Key's y coordinate is even adds "02" prefix or if it is odd adds "03" prefix
        // at the start of the Public Key's x coordinate.
        pub_key_x_byte.push(if pub_key_y_byte[0] % 2 == 0 {
            02u8
        } else {
            03u8
        });

        pub_key_x_byte.reverse();

        let compressed_pub = hex::encode(pub_key_x_byte.clone());

        compressed_pub
    }

    /// TODO: May be we wont need this. Generetes Pub Key from different method!
    /// Chop 0x prefix from key and convert it from hex to base10.
    pub fn key_to_base_ten(key: String) -> BigUint {
        let key_without_prefix = CryptoUtils::remove_prefix(key);
        let key_to_base_ten = BigUint::from_str_radix(&key_without_prefix, 16)
            .unwrap()
            .to_string()
            .parse()
            .unwrap();

        key_to_base_ten
    }

    /// TODO: Shows 04 prefix. Delete if necessary!! Changed name from account to priv_key!!
    /// Gets compressed Public Key from account object.
    pub fn get_compressed_pub_from_account(&self, priv_key: SecretKey) -> String {
        let comp_pub_key_arr =
            PublicKey::from_secret_key(&self.curve, &priv_key).serialize_uncompressed();
        let comp_pub_key = hex::encode(comp_pub_key_arr);

        comp_pub_key
    }

    /// TODO: Different from Python. Crate has own shared secret!
    /// Helper function to retrieves shared secret.
    pub fn gen_shared_secret(pub_key: PublicKey, priv_key: SecretKey) -> SharedSecret {
        let shared_secret = SharedSecret::new(&pub_key, &priv_key);

        shared_secret
    }

    /// Gets y coordinate of compressed Public Key.
    pub fn decompress_public_key(compressed_pub_key: String) -> String {
        // Hardcoded to secp256k1(eth curve).
        let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
        let p: BigInt = BigInt::from_str_radix(&p_hex, 16).unwrap();

        let b_hex = "0000000000000000000000000000000000000000000000000000000000000007";
        let b: BigInt = BigInt::from_str_radix(&b_hex, 16).unwrap();

        let x_hex = CryptoUtils::remove_prefix(compressed_pub_key);
        let x: BigInt = BigInt::from_str_radix(&x_hex.as_str(), 16).unwrap();

        // Secp256k1 is chosen in a special way so that the square root of y is y^((p+1)/4).
        let y_square = (BigInt::modpow(&x, &BigInt::from(3usize), &p) + b) % &p;
        let mut y = BigInt::modpow(&y_square, &((&p + 1usize) / 4usize), &p);

        if &y % 2 != (BigInt::from(0)) {
            y = -1 * (-y % p);
        }

        let y_hex = BigInt::to_str_radix(&y, 16);

        let mut temp_decomp_pub_key = vec!['0'; 130];
        temp_decomp_pub_key[0] = '0';
        temp_decomp_pub_key[1] = '4';

        let x_char: Vec<_> = x_hex.to_string().chars().collect();

        for i in 2..66 {
            temp_decomp_pub_key[i] = x_char[i - 2]
        }

        let y_char: Vec<_> = y_hex.to_string().chars().collect();

        for i in 66..130 {
            temp_decomp_pub_key[i] = y_char[i - 66]
        }

        let decomp_pub_key: String = temp_decomp_pub_key.into_iter().collect();

        decomp_pub_key
    }

    /// TODO Check difference between python function!
    /// Derive a Public Key from a raw Private Key.
    pub fn public_key_from_raw_priv(&self, raw_priv_key: String) -> String {
        let priv_key_to_without_prefix = CryptoUtils::remove_prefix(raw_priv_key);
        let priv_key = SecretKey::from_str(priv_key_to_without_prefix.as_str())
            .expect("Provide a valid private key!");

        let pub_key_arr = PublicKey::from_secret_key(&self.curve, &priv_key).serialize();
        let pub_key = hex::encode(pub_key_arr);

        pub_key
    }

    /// Helper function for delete prefix from beginning of the number.
    pub fn remove_prefix(data: String) -> String {
        let mut data_char = data.chars();
        data_char.next();
        data_char.next();

        let data_str = data_char.as_str().to_string();

        data_str
    }

    /// Helper function for add prefix at beginning of the number.
    pub fn add_prefix(data: &String) -> String {
        let data_biguint_byte = BigUint::from_str_radix(&data, 16).unwrap().to_bytes_be();
        let data_prefixed = ToHexPrefixed::to_hex_prefixed(data_biguint_byte);

        data_prefixed
    }
}
