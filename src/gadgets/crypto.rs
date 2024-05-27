use halo2curves::ff::PrimeField;
use halo2curves::{
    group::{Curve, Group},
    secp256k1::{Fp, Fq, Secp256k1},
};

use hex;
use num_bigint::BigUint;
use num_traits::Num;

/// Converts Private Key to base10.
pub fn key_to_base_ten(key: Fq) -> BigUint {
    let mut temp = Vec::new();

    let key_to_byte = key.to_bytes();

    for i in key_to_byte {
        temp.push(i.into())
    }

    temp.reverse();

    let key_to_hex = hex::encode(temp);

    let key_base_ten = BigUint::from_str_radix(&key_to_hex, 16)
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    println!("priv_key_base10: {:?}", key_base_ten);

    key_base_ten
}

/// Helper function to retrieves shared secret.
pub fn gen_shared_secret(priv_key: Fq, pub_key_base_ten: BigUint) -> BigUint {
    let priv_key_base_ten = key_to_base_ten(priv_key);

    let shared = priv_key_base_ten * pub_key_base_ten;

    shared
}

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

/// Gets compressed Public Key from account object.
pub fn get_compressed_pub_from_account(priv_key: Fq) -> String {
    ///// TODO will we need this? /////

    // // Make Private Key a Base10 int.
    // let priv_base_ten = key_to_base_ten(priv_key);

    //  Multiply private key by generator. Creates a Public Key.
    let pub_key = Secp256k1::generator() * priv_key;
    println!("pub_key: {:?}", pub_key);

    // Compress Public Key.
    let compressed_pub_key = compress(pub_key);

    compressed_pub_key
}

/// Gets y coordinate of compressed Public Key.
pub fn decompress_public_key(compressed_pub_key: String) -> BigUint {
    // Hardcoded to secp256k1.
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p: BigUint = BigUint::from_str_radix(&p_hex, 16).unwrap();

    let b_hex = "0000000000000000000000000000000000000000000000000000000000000007";
    let b: BigUint = BigUint::from_str_radix(&b_hex, 16).unwrap();
    println!("compressed_pub_key_x: {:?}", compressed_pub_key);

    let mut x_hex = compressed_pub_key.chars();
    x_hex.next();
    x_hex.next();
    println!("decompressed_x_hex: {:?}", x_hex.as_str());

    let x: BigUint = BigUint::from_str_radix(&x_hex.as_str(), 16).unwrap();
    println!("decompressed_x_dec: {:?}", x);

    // TODO fix parity!!!
    // Secp256k1 is chosen in a special way so that the square root of y is y^((p+1)/4).
    //let y_square = (BigUint::modpow(&x, &BigUint::from(3usize), &p) + b) % &p;
    let y_square = BigUint::pow(&x, 3u32) + b;
    let y = BigUint::modpow(&y_square, &((&p + 1usize) / 4usize), &p);

    let y_hex = BigUint::to_str_radix(&y, 16);

    println!("y_hex: {:?}", y_hex);
    println!("y: {:?}", y);

    p
}

/// TODO Check difference between python function!
/// Derive a Public Key from a raw Private Key.
pub fn public_key_from_raw_priv(raw_priv: Fq) -> String {
    let pub_key = Secp256k1::generator() * raw_priv;
    let pub_key_compressed = compress(pub_key);
    println!("pub_raw: {:?}", pub_key_compressed);
    pub_key_compressed
}
