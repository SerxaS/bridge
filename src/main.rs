mod middleware;

use eyre::Result;
use middleware::cli::arg_process;

#[tokio::main]
async fn main() -> Result<()> {
    let pub_key_from_cli = arg_process();
    Ok(())
}
