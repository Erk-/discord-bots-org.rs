//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`Requester`].
//!
//! [`Requester`]: trait.Requester.html

use crate::{
    builder::*,
    endpoints,
    model::*,
    Result,
};
use reqwest::{
    header::{AUTHORIZATION, HeaderValue},
    Client as ReqwestClient,
    Url,
};
use std::sync::Arc;

/// Struct which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the API client:
///
/// ```rust,no_run
/// use discord_bots_org::ReqwestSyncClient;
/// ```
pub struct Client {
    inner: Arc<ReqwestClient>,
}

impl Client {
    /// Creates a new client to interact with the API.
    ///
    /// This accepts an existing reqwest Client so a single HTTP client may be
    /// shared across your application.
    ///
    /// # Examples
    ///
    /// Create a new API client:
    ///
    /// ```rust
    /// extern crate reqwest;
    ///
    /// use discord_bots_org::ReqwestSyncClient as ApiClient;
    /// use reqwest::Client as ReqwestClient;
    /// use std::sync::Arc;
    ///
    /// let reqwest_client = Arc::new(ReqwestClient::new());
    /// let client = ApiClient::new(Arc::clone(&reqwest_client));
    pub fn new(reqwest_client: Arc<ReqwestClient>) -> Self {
        Self {
            inner: reqwest_client,
        }
    }

    /// Retrieves information about a bot.
    pub fn get_bot(&self, user_id: u64) -> Result<Bot> {
        let url = Url::parse(&endpoints::bot(user_id))?;

        self.inner.get(url).send()?.json().map_err(From::from)
    }

    /// Retrieves a list of bots via a search.
    pub fn get_bots<F>(&self, search: F) -> Result<SearchResponse<Bot>>
        where F: FnOnce(BotSearch) -> BotSearch {
        let params = search(BotSearch::default()).build();
        let url = Url::parse_with_params(&endpoints::bots(), params)?;

        self.inner.get(url).send()?.json().map_err(From::from)
    }

    /// Retrieves information about a bot's specific stats.
    pub fn get_bot_stats(&self, user_id: u64) -> Result<BotStats> {
        let url = Url::parse(&endpoints::bot_stats(user_id))?;

        self.inner.get(url).send()?.json().map_err(From::from)
    }

    /// Retrieve whether a user has upvoted a bot in the last 24 hours.
    ///
    /// You can use this if your bot has over 1000 votes.
    pub fn get_bot_vote_check(
        &self,
        auth: impl AsRef<str>,
        bot_id: u64,
        user_id: u64,
    ) -> Result<bool> {
        let path = endpoints::bot_vote_check(bot_id, user_id);
        let params = &[("userId", user_id.to_string())];
        let url = Url::parse_with_params(&path, params)?;

        let resp = self
            .inner
            .get(url)
            .header(AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?)
            .send()?
            .json::<ResponseUserVoted>()?;

        Ok(resp.voted == 1)
    }

    /// Retrieves information to see who has upvoted a bot.
    ///
    /// **Note**: If your bot has over 1000 votes per month, then this can not
    /// be used. Webhooks must instead be used.
    pub fn get_bot_votes(
        &self,
        auth: impl AsRef<str>,
        bot_id: u64,
    ) -> Result<BotVotes> {
        let url = Url::parse(&endpoints::bot_votes(bot_id))?;

        self
            .inner
            .get(url)
            .header(AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?)
            .send()?
            .json()
            .map_err(From::from)
    }

    /// Retrieves information about a user.
    pub fn get_user(&self, user_id: u64) -> Result<User> {
        let url = Url::parse(&endpoints::user(user_id))?;

        self.inner.get(url).send()?.json().map_err(From::from)
    }

    /// Posts a bot's shard stats.
    pub fn post_stats(
        &self,
        auth: impl AsRef<str>,
        bot_id: u64,
        stats: &ShardStats,
    ) -> Result<()> {
        let url = Url::parse(&endpoints::bot_stats(bot_id))?;

        self
            .inner
            .post(url)
            .json(stats)
            .header(AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?)
            .send()?
            .json()
            .map_err(From::from)
    }
}
