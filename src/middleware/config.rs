use serde_derive::Deserialize;
use serde_json;
use std::{fs::File, io::Read};

#[derive(Deserialize)]
pub struct ConfigIface {
    pub(crate) rpc_url: String,
    pub(crate) hss_address: String,
    pub(crate) private_key: String,
    pub(crate) socket_port: u32,
}

impl ConfigIface {
    /// TODO there is 2 fn in Python. Ask it!! I didnt give it constant!! Different!
    /// Open config file, parse contents, and ensure all expected entries are represented.
    pub fn get_config() -> Self {
        // Open config file and parse JSON.
        let mut config_file = File::open("src/config.json").expect(
            "Could not open the config file. Please provide a config.json in `src` directory.",
        );
        let mut config_str = String::new();

        config_file
            .read_to_string(&mut config_str)
            .expect("Unable to read the data!");

        // Check to ensure that all items provided in config are expected, correct type, and present.
        let config_data: ConfigIface =
            serde_json::from_str(&config_str).expect("JSON was not well-formatted!");

        config_data
    }
}
