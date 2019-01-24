//! Models mapping the Discord Bot List API.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Information about a bot.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    /// The avatar hash of the bot user.
    pub avatar: Option<String>,
    /// The certified status of the bot.
    pub certified_bot: bool,
    /// The date when the bot was approved.
    pub date: DateTime<FixedOffset>,
    /// The CDN hash of the bot's avatar if the bot has none.
    pub def_avatar: Option<String>,
    /// The long description of the bot.
    ///
    /// Can contain HTML and/or Markdown.
    #[serde(rename = "longdesc")]
    pub description_long: Option<String>,
    /// The short description of the bot.
    #[serde(rename = "shortdesc")]
    pub description_short: String,
    /// The discriminator of the bot.
    pub discriminator: String,
    /// The link to the GitHub repo of the bot.
    pub github: Option<String>,
    /// The ID of the bot.
    pub id: String,
    /// The custom bot invite URL of the bot.
    pub invite: Option<String>,
    /// The library of the bot.
    pub lib: String,
    /// The owners of the bot. First one in the array is the main owner.
    pub owners: Vec<String>,
    /// The amount of upvotes the bot has.
    pub points: u64,
    /// The prefix of the bot.
    pub prefix: String,
    /// The support server invite code of the bot.
    pub support: Option<String>,
    /// The tags of the bot.
    pub tags: Vec<String>,
    /// The username of the bot.
    pub username: String,
    /// The vanity URL of the bot.
    pub vanity: Option<String>,
    /// The website URL of the bot.
    pub website: Option<String>,
}

/// Information about a bot's statistics.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotStats {
    /// The amount of servers the bot is in.
    pub server_count: Option<u64>,
    /// The amount of servers the bot is in per shard.
    ///
    /// This is always present, but may be 0-length.
    pub shards: Vec<u64>,
    /// The amount of shards a bot has.
    pub shard_count: Option<u64>,
}

/// Information about who has voted for a bot.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BotVotes {
    /// A list of IDs of the Discord users who have voted for a bot.
    Ids(Vec<u64>),
    /// A list of user objects of the Discord users who have voted for a bot.
    Users(Vec<DiscordUser>),
}

/// Information about a Discord user.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordUser {
    /// The avatar hash of the user's avatar.
    pub avatar: Option<String>,
    /// The discriminator of the user.
    pub discriminator: u16,
    /// The ID of the user.
    pub id: String,
    /// The username of the user.
    pub username: String,
}

#[derive(Deserialize)]
pub(crate) struct ResponseUserVoted {
    pub voted: u8,
}

/// Information about a search response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<T> {
    /// The length of the results vector.
    pub count: u64,
    /// The limit used.
    pub limit: u64,
    /// The offset used.
    pub offset: u64,
    /// The matching results.
    pub results: Vec<T>,
    /// The total number of results matching the search.
    pub total: u64,
}

/// Information about one or more shards, used to update a bot's sharding
/// stats.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ShardStats {
    /// Used to post the cumulative guild information for all of the bot.
    Cumulative {
        /// The total number of shards in use.
        shard_count: Option<u64>,
        /// The total number of guilds across the entire bot.
        total: u64,
    },
    /// Used to post the guild information for a single shard.
    Shard {
        /// The total number of guilds in the shard.
        guild_count: u16,
        /// The total number of shards in use.
        shard_count: u64,
        /// The ID of the shard being posted for.
        shard_id: u64,
    },
    /// Used to post the guild information for all shards.
    ///
    /// Each vector index is the shard ID mapped to the number of guilds in the
    /// shard.
    Shards(Vec<u64>),
}

/// Social information about a user.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Social {
    /// The GitHub username of the user.
    pub github: String,
    /// The Instagram username of the user.
    pub instagram: String,
    /// The Reddit username of the user.
    pub reddit: String,
    /// The Twitter username of the user.
    pub twitter: String,
    /// The YouTube username of the user.
    pub youtube: String,
}

/// Information about a user.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The admin status of the user.
    pub admin: bool,
    /// The avatar hash of the user's avatar.
    pub avatar: Option<String>,
    /// The banner image URL of the user.
    pub banner: Option<String>,
    /// The bio of the user.
    pub bio: Option<String>,
    /// The certified status of the user.
    pub certified_dev: bool,
    /// The custom hex colour of the user.
    #[serde(rename = "color")]
    pub colour: Option<String>,
    /// The CDN hash of the user's avatar if the user has none.
    pub def_avatar: Option<String>,
    /// The discriminator of the user.
    pub discriminator: String,
    /// The ID of the user.
    pub id: String,
    /// The mod status of the user.
    #[serde(rename = "mod")]
    pub mod_: bool,
    /// The user's social information.
    #[serde(default)]
    pub social: Social,
    /// Whether the use is a support of the website.
    pub supporter: bool,
    /// The username of the user.
    pub username: String,
    /// The website moderator status of the user.
    pub web_mod: bool,
}
