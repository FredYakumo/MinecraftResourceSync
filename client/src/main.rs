mod module;
use module::client_module::client_run;
use tokio;

#[tokio::main]
async fn main() {
    client_run().await;
}