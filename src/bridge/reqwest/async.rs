//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`Requester`].
//!
//! [`Requester`]: trait.Requester.html

use crate::{
    endpoints,
    model::*,
    Result,
};
use futures::compat::Future01CompatExt;
use reqwest::{
    r#async::Client as ReqwestClient,
    header::{AUTHORIZATION, HeaderValue},
    Url,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// Struct which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the API client:
///
/// ```rust,no_run
/// use discord_bots_org::bridge::reqwest::r#async::Client;
/// ```
#[derive(Clone, Debug)]
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
    /// use discord_bots_org::ReqwestAsyncClient as ApiClient;
    /// use reqwest::r#async::Client as ReqwestClient;
    /// use std::sync::Arc;
    ///
    /// let reqwest_client = Arc::new(ReqwestClient::new());
    /// let api_client = ApiClient::new(Arc::clone(&reqwest_client));
    pub fn new(reqwest_client: Arc<ReqwestClient>) -> Self {
        Self {
            inner: reqwest_client,
        }
    }

    /// Retrieves information about a bot.
    pub async fn get_bot(&self, user_id: u64) -> Result<Bot> {
        await!(self.get(Url::parse(&endpoints::bot(user_id))?))
    }

    /// Retrieves a list of bots via a search.
    pub async fn get_bots<'a>(
        &'a self,
        params: Vec<(&'a str, String)>,
    ) -> Result<SearchResponse<Bot>> {
        await!(self.get(Url::parse_with_params(&endpoints::bots(), params)?))
    }

    /// Retrieves information about a bot's specific stats.
    pub async fn get_bot_stats(&self, user_id: u64) -> Result<BotStats> {
        await!(self.get(Url::parse(&endpoints::bot_stats(user_id))?))
    }

    /// Retrieve whether a user has upvoted a bot in the last 24 hours.
    ///
    /// You can use this if your bot has over 1000 votes.
    pub async fn get_bot_vote_check<'a>(
        &'a self,
        auth: impl AsRef<str> + 'a,
        bot_id: u64,
        user_id: u64,
    ) -> Result<bool> {
        let path = endpoints::bot_vote_check(bot_id, user_id);
        let params = &[("userId", user_id.to_string())];
        let url = Url::parse_with_params(&path, params)?;
        let (k, v) = (AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?);

        let mut resp = await!(self.inner.get(url).header(k, v).send().compat())?;
        let body = await!(resp.json::<ResponseUserVoted>().compat())?;

        Ok(body.voted == 1)
    }

    /// Retrieves information to see who has upvoted a bot.
    ///
    /// **Note**: If your bot has over 1000 votes per month, then this can not
    /// be used. Webhooks must instead be used.
    pub async fn get_bot_votes<'a>(
        &'a self,
        auth: impl AsRef<str> + 'a,
        bot_id: u64,
    ) -> Result<BotVotes> {
        let url = Url::parse(&endpoints::bot_votes(bot_id))?;
        let (k, v) = (AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?);
        let mut resp = await!(self.inner.get(url).header(k, v).send().compat())?;

        await!(resp.json().compat()).map_err(From::from)
    }

    /// Retrieves information about a user.
    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        await!(self.get(Url::parse(&endpoints::user(user_id))?))
    }

    /// Posts a bot's shard stats.
    pub async fn post_stats<'a>(
        &'a self,
        auth: impl AsRef<str> + 'a,
        bot_id: u64,
        stats: &'a ShardStats,
    ) -> Result<()> {
        let url = Url::parse(&endpoints::bot_stats(bot_id))?;
        let (k, v) = (AUTHORIZATION, HeaderValue::from_str(auth.as_ref())?);
        await!(self
            .inner
            .post(url)
            .header(k, v)
            .json(stats)
            .send()
            .compat())?;

        Ok(())
    }

    async fn get<'a, T: DeserializeOwned>(&'a self, url: Url) -> Result<T> {
        let mut resp = await!(self.inner.get(url).send().compat())?;

        await!(resp.json::<T>().compat()).map_err(From::from)
    }
}
