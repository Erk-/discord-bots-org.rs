use std::{
    collections::HashMap,
    fmt::Display,
};

/// Builder to filter bot results.
///
/// # Examples
///
/// Create a filter to offset 10 bots and search for 20:
///
/// ```rust
/// use discord_bots_org::builder::BotSearch;
///
/// let mut search = BotSearch::new();
/// search.offset(10).limit(20);
/// let params = search.build();
/// ```
pub struct BotSearch(HashMap<&'static str, String>);

impl BotSearch {
    /// Creates a new builder for filtering bots in a search.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds into the URL query params.
    pub fn build(self) -> Vec<(&'static str, String)> {
        self.0.into_iter().map(|(k, v)| (k, v)).collect()
    }

    /// The amount of bots to return, used for pagination.
    ///
    /// The maximum value is 500.
    pub fn limit(&mut self, mut limit: u16) -> &mut Self {
        if limit > 500 {
            limit = 500;
        }

        self.0.insert("limit", limit.to_string());

        self
    }

    /// The amount of bots to skip, used for pagination.
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.0.insert("offset", offset.to_string());

        self
    }

    /// A search query string.
    pub fn search(&mut self, query: impl Into<String>) -> &mut Self {
        self._search(query.into())
    }

    fn _search(&mut self, query: String) -> &mut Self {
        self.0.insert("search", query);

        self
    }

    /// The field to sort by.
    pub fn sort(&mut self, field: impl Display, ascending: bool) -> &mut Self {
        let prefix = if ascending {
            ""
        } else {
            "-"
        };

        self.0.insert("sort", format!("{}{}", prefix, field));

        self
    }
}

impl Default for BotSearch {
    fn default() -> Self {
        BotSearch(HashMap::with_capacity(4))
    }
}

#[cfg(test)]
mod tests {
    use super::BotSearch;

    #[test]
    fn test_field() {
        let mut search = BotSearch::new();
        search.limit(10).offset(20).search("hi").sort("b", false);

        assert_eq!(search.0.get(&"limit").unwrap(), "10");
        assert_eq!(search.0.get(&"offset").unwrap(), "20");
        assert_eq!(search.0.get(&"search").unwrap(), "hi");
        assert_eq!(search.0.get(&"sort").unwrap(), "-b");
    }
}
