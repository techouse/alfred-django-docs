use std::process::Command;

use serde_json::Value;

fn workflow_binary() -> &'static str {
    env!("CARGO_BIN_EXE_alfred_django_docs")
}

#[test]
fn unknown_argument_returns_one_alfred_error_item_and_exit_two()
-> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(workflow_binary()).arg("--unknown").output()?;

    assert_eq!(output.status.code(), Some(2));
    let rendered: Value = serde_json::from_slice(&output.stdout)?;
    let items = rendered["items"]
        .as_array()
        .expect("items must be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["title"], "unknown argument: --unknown");
    Ok(())
}

#[test]
fn empty_algolia_application_id_returns_config_error_item_and_exit_one()
-> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(workflow_binary())
        .args(["--verbose", "--query", "runtime-error"])
        .env("ALGOLIA_APPLICATION_ID", "")
        .env("ALGOLIA_SEARCH_ONLY_API_KEY", "key")
        .env("ALGOLIA_SEARCH_INDEX", "django")
        .output()?;

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("ALGOLIA_APPLICATION_ID must not be empty"));

    let rendered: Value = serde_json::from_slice(&output.stdout)?;
    let items = rendered["items"]
        .as_array()
        .expect("items must be an array");
    assert!(
        items
            .iter()
            .any(|item| { item["title"] == "ALGOLIA_APPLICATION_ID must not be empty" })
    );
    Ok(())
}
