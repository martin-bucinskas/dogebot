mod cli;
mod auth;
mod client;

use crate::opt::Opt;
use std::env;
use log::{info, debug};
use slack_rust as slack;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::channels::channel::Channel;
use slack::http_client::default_client;

#[macro_use] extern crate rocket;

#[get("/oauth/callback?<code>&<state>")]
async fn oauth_callback(code: &str, state: &str) {
    debug!("Received: {}, {}", code, state);
    info!("Exchanging code for access token...");

    let token = auth::slack_auth::exchange_code_for_access_token(code).await.unwrap();

    let slack_api_client = default_client();
    let param = PostMessageRequest {
        channel: "general".to_string(),
        text: Some("wow much doge so cereal".to_string()),
        ..Default::default()
    };
    let response = post_message(&slack_api_client, &param, &token.access_token)
      .await
      .expect("api call error");
    info!("Slack response: {:?}", response);
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    let opt = cli::opt::Opt::from_args();
    let is_debug_mode = opt.is_debug_enabled();

    if *is_debug_mode {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    rocket::build()
      .mount("/", routes![oauth_callback])
      .launch()
      .await
}
