use std::io::Write;
use log::{error, info};
use std::str;
use dapr::Client;
use dapr::client::{GetSecretResponse, TonicClient};
use rocket::futures::SinkExt;
use serde::Serialize;
use bincode;

use reqwest;
use slack_rust::team::teams::Team;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SlackTeam {
  name: Box<str>,
  id: Box<str>,
}

// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// struct SlackEnterprise {
//   name: Box<str>,
//   id: Box<str>,
// }
//
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// struct SlackAuthedUser {
//   id: Box<str>,
//   scope: Box<str>,
//   access_token: Box<str>,
//   token_type: Box<str>,
// }

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SlackToken {
  ok: bool,
  pub(crate) access_token: Box<str>,
  token_type: Box<str>,
  scope: Box<str>,
  bot_user_id: Box<str>,
  app_id: Box<str>,
  team: SlackTeam,
  // enterprise: SlackEnterprise,
  // authed_user: SlackAuthedUser,
}

pub async fn exchange_code_for_access_token(code: &str) -> Option<SlackToken> {

  let port: u16 = std::env::var("DAPR_GRPC_PORT").unwrap().parse().unwrap();
  let addr = format!("https://127.0.0.1:{}", port);
  let mut dapr_client: Client::<TonicClient> = dapr::Client::<TonicClient>::connect(addr).await.unwrap();

  let client_id_secret = &dapr_client.get_secret("dogebot-secret-store", "slackClientId").await.unwrap();
  let client_id = client_id_secret.data.get("slackClientId").expect("unable to fetch client_id from secrets");

  let client_secret_secret = &dapr_client.get_secret("dogebot-secret-store", "slackClientSecret").await.unwrap();
  let client_secret = client_secret_secret.data.get("slackClientSecret").expect("unable to fetch client_secret from secrets");

  let redirect_uri_secret = &dapr_client.get_secret("dogebot-secret-store", "slackRedirectUri").await.unwrap();
  let redirect_uri = redirect_uri_secret.data.get("slackRedirectUri").expect("unable to fetch redirect_uri from secrets");

  let params = [
    ("client_id", client_id),
    ("client_secret", client_secret),
    ("code", &code.to_string()),
    ("grant_type", &"authorization_code".to_string())
  ];

  let client = reqwest::Client::new();
  let res = client.post("https://slack.com/api/oauth.v2.access")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .form(&params)
    .send()
    .await;

  match res {
    Ok(response) => {
      let token: SlackToken = response.json().await.unwrap();
      let json_token = serde_json::to_string(&token).unwrap();
      let state = dapr_client.save_state("dogebot-state-store", vec![("slack-token", String::from(&json_token).into_bytes())]).await;
      Option::Some(token)
    }
    Err(err) => {
      error!("Failed exchanging code for token: {:?}", err);
      Option::None
    }
  }
}
