#[cfg(test)]
mod tests {
    use crate::gadgets::crypto;
    use halo2curves::{ff::Field, secp256k1::Fq};
    use rand::thread_rng;

    #[test]
    fn test_crypto() {
        // Creates a Private Key from random Fq element.
        let rng = thread_rng();
        let priv_key = Fq::random(rng);
        println!("priv key: {:?}", priv_key);

        let compressed_pub_key = crypto::get_compressed_pub_from_account(priv_key);
        let decoompress_public_key = crypto::decompress_public_key(compressed_pub_key);
        let public_key_from_raw_priv = crypto::public_key_from_raw_priv(priv_key);
        let key_to_base_ten = crypto::key_to_base_ten(priv_key);
    }
}
