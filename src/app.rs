use alfred_workflow_rs::{Icon, Item, ItemText};
use anyhow::Result;
use url::Url;

use crate::models::SearchResult;

/// Builds the placeholder shown before the user enters a search query.
pub fn placeholder_item() -> Item {
    Item::new("Search the Django docs...").set_icon(Icon::new("icon.png"))
}

/// Converts ranked Django search results into Alfred items in provider order.
pub fn items_from_results(results: &[SearchResult]) -> Result<Vec<Item>> {
    results.iter().map(item_from_result).collect()
}

/// Builds the Google fallback shown when Algolia returns no hits.
pub fn google_fallback_item(query: &str) -> Result<Item> {
    let url = Url::parse_with_params(
        "https://www.google.com/search",
        [("q", format!("Django {query}"))],
    )?;

    Ok(Item::builder("No matching answers found")
        .subtitle("Shall I try and search Google?")
        .arg(url.as_str())
        .text(ItemText::new(url.as_str()))
        .quick_look_url(url.as_str())
        .icon(Icon::new("google.png"))
        .valid(true)
        .build()?)
}

fn item_from_result(result: &SearchResult) -> Result<Item> {
    let subtitle = if result.content.is_empty() {
        result.id.clone()
    } else {
        truncate_content(&result.content)
    };

    Ok(Item::builder(result.pretty_title())
        .uid(&result.object_id)
        .subtitle(subtitle)
        .arg(&result.permalink)
        .text(ItemText::new(&result.id).with_large_type(&result.id))
        .quick_look_url(&result.permalink)
        .icon(Icon::new("icon.png"))
        .valid(true)
        .build()?)
}

fn truncate_content(content: &str) -> String {
    if content.chars().count() <= 75 {
        return content.to_owned();
    }

    content.chars().take(72).chain("...".chars()).collect()
}

#[cfg(test)]
#[path = "tests/app.rs"]
mod tests;
