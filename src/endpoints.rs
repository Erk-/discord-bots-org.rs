// This module can be "dead" if no features are enabled.
#![cfg_attr(not(any(
    feature = "reqwest-async-support",
    feature = "reqwest-sync-support"
)), allow(dead_code))]

/// API URI base.
const BASE: &str = "https://discordbots.org/api";

pub fn bot(id: u64) -> String {
    format!("{}/bots/{}", BASE, id)
}

pub fn bot_stats(id: u64) -> String {
    format!("{}/bots/{}/stats", BASE, id)
}

pub fn bot_vote_check(bot_id: u64, user_id: u64) -> String {
    format!("{}/bots/{}/check?userId={}", BASE, bot_id, user_id)
}

pub fn bot_votes(id: u64) -> String {
    format!("{}/bots/{}/votes", BASE, id)
}

pub fn bots() -> String {
    format!("{}/bots", BASE)
}

pub fn user(id: u64) -> String {
    format!("{}/users/{}", BASE, id)
}

pub fn widget(id: u64) -> String {
    format!("{}/widget/{}.svg", BASE, id)
}
