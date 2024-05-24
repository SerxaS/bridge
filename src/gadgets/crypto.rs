use halo2curves::secp256k1::{Fq, Secp256k1};
use hex;
use num_bigint::BigUint;
use num_traits::Num;

/// Helper function for compresses the Public Key's x & y coordinates into common format.
fn compress(pub_key: Secp256k1) -> String {
    // Byte representation of x & y coordinates.
    let mut pub_key_x_byte = pub_key.x.to_bytes().to_vec();
    let pub_key_y_byte = pub_key.y.to_bytes();

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

/// Converts Private Key to base10. 
pub fn key_to_base_ten(key: Fq) -> BigUint {
    let mut temp_vec = Vec::new();

    let key_to_byte = key.to_bytes();

    for i in key_to_byte {
        temp_vec.push(i.into())
    }

    temp_vec.reverse();

    let key_to_hex = hex::encode(temp_vec);

    let key_base_ten = BigUint::from_str_radix(&key_to_hex, 16)
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    println!("priv key base10: {:?}", key_base_ten);
    key_base_ten
}

/// Gets compressed Public Key from account object.
pub fn get_compressed_pub_from_account(priv_key: Fq) -> String {
    ///// TODO will we need this? /////

    // // Make Private Key a Base10 int.
    // let priv_base_ten = key_to_base_ten(priv_key);

    //  Multiply private key by generator. Creates a Public Key.
    let pub_key = Secp256k1::generator() * priv_key;
    println!("pub key: {:?}", pub_key);

    // Compress Public Key.
    let compressed_pub_key = compress(pub_key);

    compressed_pub_key
}

/// Helper function to retrieves shared secret.
pub fn gen_shared_secret(priv_key: Fq, pub_key_base_ten: BigUint) -> BigUint {
    let priv_key_base_ten = key_to_base_ten(priv_key);

    let shared = priv_key_base_ten * pub_key_base_ten;

    shared
}

/// Gets y coordinate of compressed Public Key.
pub fn decompress_public_key(compressed_pub_key: String) -> BigUint {
    // Hardcoded to secp256k1.
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p: BigUint = BigUint::from_str_radix(&p_hex, 16).unwrap();

    let b_hex = "0000000000000000000000000000000000000000000000000000000000000007";
    let b: BigUint = BigUint::from_str_radix(&b_hex, 16).unwrap();
    println!("compressed pub key: {:?}", compressed_pub_key);
    let mut x_hex = compressed_pub_key.chars();
    x_hex.next();
    x_hex.next();
    println!("{:?}", x_hex.as_str());

    let x: BigUint = BigUint::from_str_radix(&x_hex.as_str(), 16).unwrap();
    println!("{:?}", x);

    /// TODO fix parity!!!
    // Secp256k1 is chosen in a special way so that the square root of y is y^((p+1)/4).
    let y_square = (BigUint::pow(&x, 3) + b) % p.clone();
    let y = BigUint::modpow(&y_square, &((p.clone() + 1usize) / 4usize), &p);

    println!("{:?}", y);

    y
    // let y_str = y.to_str_radix(16);
    // let mut y_str_byte = y_str.as_bytes().to_vec();
    // println!("{:?}", y_str_byte);

    // y_str_byte.reverse();

    // let y_arr: [u8; 64] = y_str_byte.try_into().unwrap_or_else(|v: Vec<u8>| {
    //     panic!("Expected a Vec of length {} but it was {}", 64, v.len())
    // });

    // let y_str_fr = Fq::from_uniform_bytes(&y_arr);
}

/// TODO Check difference between python function!
/// Derive a Public Key from a raw Private Key.
pub fn public_key_from_raw_priv(raw_priv: Fq) -> String {
    let pub_key = Secp256k1::generator() * raw_priv;
    let pub_key_compressed = compress(pub_key);
    println!("{:?}", pub_key_compressed);
    pub_key_compressed
}
