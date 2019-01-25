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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot() {
        assert_eq!(bot(1), "https://discordbots.org/api/bots/1");
    }

    #[test]
    fn test_bot_stats() {
        assert_eq!(bot_stats(1), "https://discordbots.org/api/bots/1/stats");
    }

    #[test]
    fn test_bot_vote_check() {
        assert_eq!(
            bot_vote_check(1, 2),
            "https://discordbots.org/api/bots/1/check?userId=2",
        );
    }

    #[test]
    fn test_bot_votes() {
        assert_eq!(bot_votes(1), "https://discordbots.org/api/bots/1/votes");
    }

    #[test]
    fn test_bots() {
        assert_eq!(bots(), "https://discordbots.org/api/bots");
    }

    #[test]
    fn test_user() {
        assert_eq!(user(1), "https://discordbots.org/api/users/1");
    }

    #[test]
    fn test_widget() {
        assert_eq!(widget(1), "https://discordbots.org/api/widget/1.svg");
    }
}
