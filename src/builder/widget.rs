//! Types for generating widget embed URLs.

use crate::{endpoints, Result};
use std::collections::HashMap;
use url::Url;

struct Widget(u64, HashMap<&'static str, String>);

impl Widget {
    fn build(self) -> Result<String> {
        let uri = endpoints::widget(self.0);
        let params = self
            .1
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<_>>();

        let url = Url::parse_with_params(&uri, params)?;

        Ok(url.into_string())
    }

    fn insert(&mut self, k: &'static str, v: impl Into<String>) -> &mut Self {
        self._insert(k, v.into())
    }

    fn _insert(&mut self, k: &'static str, v: String) -> &mut Self {
        self.1.insert(k, v);

        self
    }
}

/// Type-safe and guarenteed method of making a large widget.
pub struct LargeWidget(Widget);

impl LargeWidget {
    /// Creates a new builder for making a large widget.
    pub fn new(bot_id: u64) -> Self {
        LargeWidget(Widget(bot_id, HashMap::new()))
    }

    /// Builds into a valid URL.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidUrl`] if one of the query parameters is invalid.
    ///
    /// [`Error::InvalidUrl`]: ../enum.Error.html#variant.InvalidUrl
    pub fn build(self) -> Result<String> {
        self.0.build()
    }

    /// Sets the top color of the widget.
    pub fn top_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("topcolor", value);

        self
    }

    /// Sets the middle color of the widget.
    pub fn middle_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("middlecolor", value);

        self
    }

    /// Sets the username color of the widget.
    pub fn username_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("usernamecolor", value);

        self
    }

    /// Sets the certified color of the widget.
    pub fn certified_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("certifiedcolor", value);

        self
    }

    /// Sets the data color of the widget.
    pub fn data_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("datacolor", value);

        self
    }

    /// Sets the label color of the widget.
    pub fn label_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("labelcolor", value);

        self
    }
}

/// Type-safe and guarenteed method of making a small widget.
pub struct SmallWidget(Widget);

impl SmallWidget {
    /// Creates a new builder for making a small widget.
    pub fn new(bot_id: u64) -> Self {
        SmallWidget(Widget(bot_id, HashMap::new()))
    }

    /// Builds into a valid URL.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidUrl`] if one of the query parameters is invalid.
    ///
    /// [`Error::InvalidUrl`]: ../enum.Error.html#variant.InvalidUrl
    pub fn build(self) -> Result<String> {
        self.0.build()
    }

    /// Sets the background color of the widget.
    pub fn avatar_background(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("avatarbg", value);

        self
    }

    /// Sets the left color of the widget.
    pub fn left_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("leftcolor", value);

        self
    }

    /// Sets the right color of the widget.
    pub fn right_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("rightcolor", value);

        self
    }

    /// Sets the left text color of the widget.
    pub fn left_text_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("lefttextcolor", value);

        self
    }

    /// Sets the right text color of the widget.
    pub fn right_text_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.insert("righttextcolor", value);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;
    use super::SmallWidget;

    #[test]
    fn test_color() -> Result<()> {
        let mut widget = SmallWidget::new(270_198_738_570_444_801);
        widget.left_color("FF0000").left_text_color("FFFFFF");

        // The ordering of `url`'s parsed query parameters isn't always
        // reproducable.
        let url = widget.build()?;
        assert!(url.contains("lefttextcolor=FFFFFF"));
        assert!(url.contains("leftcolor=FF0000"));
        assert!(url.starts_with("https://discordbots.org/api/widget/270198738570444801.svg?"));

        Ok(())
    }
}
