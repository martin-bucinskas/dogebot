use std::sync::Mutex;
use dapr;
use dapr::Client;
use dapr::client::TonicClient;

pub struct SharedDaprClient {
  pub(crate) client: Mutex<Client<TonicClient>>
}

pub async fn create_client() -> Result<SharedDaprClient, Box<dyn std::error::Error>> {
  let port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
  let addr = format!("https://127.0.0.1:{}", port);
  let client = dapr::Client::<TonicClient>::connect(addr).await?;

  Ok(SharedDaprClient {
    client: Mutex::new(client)
  })
}