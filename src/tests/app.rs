use super::*;
use alfred_workflow_rs::Item;
use serde_json::json;

fn result(id: &str, title: &str, content: &str) -> SearchResult {
    SearchResult {
        object_id: format!("object-{id}"),
        categories: vec!["docs".to_owned()],
        content: content.to_owned(),
        id: id.to_owned(),
        permalink: format!("https://docs.djangoproject.com/en/5.2/{id}/"),
        title: title.to_owned(),
        version: json!(5.2).as_number().cloned().expect("number"),
    }
}

#[test]
fn every_django_prefix_gets_its_historical_title_label() -> Result<()> {
    let cases = [
        ("django.conf", "django.conf", "django.conf"),
        ("setting-debug", "Debug", "Debug [setting]"),
        ("templatefilter-add", "Add", "Add [template filter]"),
        ("fieldlookup-exact", "Exact", "Exact [field lookup]"),
        ("templatetag-for", "For", "For [template tag]"),
        ("cmdoption-runserver", "Runserver", "Runserver [cmd option]"),
        (
            "envvar-django-settings",
            "Django settings",
            "Django settings [env var]",
        ),
        ("ordinary", "Ordinary", "Ordinary"),
    ];

    let results = cases
        .iter()
        .map(|(id, title, _)| result(id, title, "content"))
        .collect::<Vec<_>>();
    let items = items_from_results(&results)?;

    for (item, (_, _, expected)) in items.iter().zip(cases) {
        assert_eq!(item.title(), expected);
    }
    Ok(())
}

#[test]
fn empty_content_uses_id_as_subtitle() -> Result<()> {
    let item = items_from_results(&[result("django.conf", "django.conf", "")])?.remove(0);

    assert_eq!(item.subtitle(), Some("django.conf"));
    Ok(())
}

#[test]
fn content_is_truncated_to_75_unicode_scalars() -> Result<()> {
    let item = items_from_results(&[result("long", "Long", &"é".repeat(80))])?.remove(0);
    let subtitle = item.subtitle().expect("subtitle must be present");

    assert_eq!(subtitle.chars().count(), 75);
    assert_eq!(subtitle, format!("{}...", "é".repeat(72)));
    Ok(())
}

#[test]
fn item_preserves_provider_order_and_metadata() -> Result<()> {
    let first = result("first", "First", "content");
    let mut second = result("second", "Second", "content");
    second.object_id = "second-object".to_owned();

    let items = items_from_results(&[first, second])?;

    assert_eq!(
        items.iter().map(Item::uid).collect::<Vec<_>>(),
        vec![Some("object-first"), Some("second-object")]
    );
    assert_eq!(
        items[0].arg(),
        Some("https://docs.djangoproject.com/en/5.2/first/")
    );
    assert_eq!(items[0].quick_look_url(), items[0].arg());
    assert_eq!(items[0].icon().map(|icon| icon.path()), Some("icon.png"));
    assert!(items[0].valid());
    assert_eq!(items[0].text().map(|text| text.copy()), Some("first"));
    assert_eq!(
        items[0].text().and_then(|text| text.large_type()),
        Some("first")
    );
    Ok(())
}

#[test]
fn titles_and_content_are_not_html_decoded() -> Result<()> {
    let item =
        items_from_results(&[result("ordinary", "A &amp; title", "A &amp; content")])?.remove(0);

    assert_eq!(item.title(), "A &amp; title");
    assert_eq!(item.subtitle(), Some("A &amp; content"));
    Ok(())
}

#[test]
fn google_fallback_encodes_query_and_is_selectable() -> Result<()> {
    let item = google_fallback_item("request body")?;

    assert_eq!(
        item.arg(),
        Some("https://www.google.com/search?q=Django+request+body")
    );
    assert_eq!(item.quick_look_url(), item.arg());
    assert_eq!(item.text().map(|text| text.copy()), item.arg());
    assert_eq!(item.icon().map(|icon| icon.path()), Some("google.png"));
    assert!(item.valid());
    Ok(())
}

#[test]
fn placeholder_is_not_selectable() {
    let item = placeholder_item();

    assert_eq!(item.title(), "Search the Django docs...");
    assert_eq!(item.icon().map(|icon| icon.path()), Some("icon.png"));
    assert!(!item.valid());
}
