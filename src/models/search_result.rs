use serde::Deserialize;
use serde_json::Number;

/// An Algolia result returned by the Django documentation index.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchResult {
    /// Algolia object identifier used as Alfred's stable UID.
    #[serde(rename = "objectID")]
    pub object_id: String,
    /// Search index categories retained for compatibility with the provider payload.
    pub categories: Vec<String>,
    /// Searchable documentation content.
    pub content: String,
    /// Django documentation anchor or record identifier.
    pub id: String,
    /// Documentation URL opened by Alfred.
    pub permalink: String,
    /// Provider title, retained exactly as returned.
    pub title: String,
    /// Numeric provider version retained for complete historical deserialization.
    pub version: Number,
}

impl SearchResult {
    /// Applies Django's historical ID-prefix labels without changing the title text.
    pub fn pretty_title(&self) -> String {
        let suffix = if self.id.starts_with("django.") {
            return self.id.clone();
        } else if self.id.starts_with("setting-") {
            "setting"
        } else if self.id.starts_with("templatefilter-") {
            "template filter"
        } else if self.id.starts_with("fieldlookup-") {
            "field lookup"
        } else if self.id.starts_with("templatetag-") {
            "template tag"
        } else if self.id.starts_with("cmdoption-") {
            "cmd option"
        } else if self.id.starts_with("envvar-") {
            "env var"
        } else {
            return self.title.clone();
        };

        format!("{} [{suffix}]", self.title)
    }
}

/// Minimal subset of an Algolia single-index search response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchResponse {
    /// Search results in provider-defined ranking order.
    pub hits: Vec<SearchResult>,
}
