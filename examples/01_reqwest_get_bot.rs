extern crate discord_bots_org;
extern crate reqwest;

use discord_bots_org::ReqwestSyncClient as ApiClient;
use reqwest::Client as ReqwestClient;
use std::sync::Arc;

fn main() {
    // Create the reqwest Client.
    let reqwest_client = Arc::new(ReqwestClient::new());

    // Create the API client.
    let client = ApiClient::new(reqwest_client);

    // Request the bot information.
    let bot = client.get_bot(136_107_769_680_887_808).expect("Err getting bot");

    println!("The bot's name is: {}", bot.username);
}
