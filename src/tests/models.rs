use crate::models::{SearchResponse, SearchResult};
use anyhow::Result;
use serde_json::json;

#[test]
fn search_result_requires_all_historical_fields() {
    let missing_version = json!({
        "objectID": "docs",
        "categories": [],
        "content": "content",
        "id": "docs",
        "permalink": "https://docs.djangoproject.com/en/5.2/docs/",
        "title": "Docs"
    });
    let missing_categories = json!({
        "objectID": "docs",
        "content": "content",
        "id": "docs",
        "permalink": "https://docs.djangoproject.com/en/5.2/docs/",
        "title": "Docs",
        "version": 5.2
    });

    assert!(serde_json::from_value::<SearchResult>(missing_version).is_err());
    assert!(serde_json::from_value::<SearchResult>(missing_categories).is_err());
}

#[test]
fn search_response_keeps_provider_order_and_numeric_version() -> Result<()> {
    let response: SearchResponse = serde_json::from_value(json!({
        "hits": [
            {
                "objectID": "first",
                "categories": ["guide"],
                "content": "First content",
                "id": "first",
                "permalink": "https://docs.djangoproject.com/en/5.2/first/",
                "title": "First",
                "version": 5.2
            },
            {
                "objectID": "second",
                "categories": [],
                "content": "",
                "id": "second",
                "permalink": "https://docs.djangoproject.com/en/5.2/second/",
                "title": "Second",
                "version": 5
            }
        ]
    }))?;

    assert_eq!(
        response
            .hits
            .iter()
            .map(|hit| hit.object_id.as_str())
            .collect::<Vec<_>>(),
        vec!["first", "second"]
    );
    assert_eq!(
        response.hits[0].version,
        json!(5.2).as_number().cloned().expect("number")
    );
    Ok(())
}
