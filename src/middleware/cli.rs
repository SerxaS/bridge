use clap::{command, Arg};

pub fn arg_process() -> String {
    // Add argument to parser.
    let arg_data = command!()
        .about("CLI for Semaphore Network Registration")
        .arg(Arg::new("Public Key").short('p').help(
            "Uncompressed Public Key for registration. The Key must contain 130 digits with 04 prefix!",
        ))
        .get_matches();

    let pub_key_for_register = arg_data
        .get_one::<String>("Public Key")
        .expect("Uncompressed Public Key must contain 130 digits with 04 prefix for Semaphore Network Registration!")
        .to_string();

    pub_key_for_register
}

/// TODO: Dont understand usage!
pub fn trigger(public_key: String) {}
