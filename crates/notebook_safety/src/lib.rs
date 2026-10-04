//! Lossless merging and validation of notebook edits before a file is written.
use anyhow::{Context as _, Result, bail, ensure};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// A recovery snapshot retains the exact disk version it was based on. Never
/// restore a draft over an independently changed file without resolving it.
pub fn recovery_record(original_text: &str, draft_text: &str) -> Result<String> {
    let draft: Value = serde_json::from_str(draft_text)?;
    validate(&draft)?;
    Ok(serde_json::to_string(&serde_json::json!({
        "version": 1, "original_text": original_text, "draft_text": draft_text
    }))?)
}

pub fn restore_recovery(record: &str, current_text: &str) -> Result<Option<String>> {
    let record: Value = serde_json::from_str(record)?;
    ensure!(
        record["version"] == 1,
        "Unsupported recovery snapshot version"
    );
    let draft = record["draft_text"]
        .as_str()
        .context("Missing recovery draft")?;
    let draft_json: Value = serde_json::from_str(draft)?;
    validate(&draft_json)?;
    if serde_json::from_str::<Value>(current_text).is_ok_and(|current| current == draft_json) {
        return Ok(None);
    }
    ensure!(
        record["original_text"].as_str() == Some(current_text),
        "Notebook changed on disk; recovery snapshot was retained for manual reconciliation"
    );
    Ok(Some(draft.to_owned()))
}

/// Preserve fields that the editor does not understand, while applying its edits.
/// `preserve_outputs` identifies cells whose outputs have not been run or cleared.
pub fn prepare_save(
    original: &Value,
    edited: Value,
    preserve_outputs: &HashSet<String>,
) -> Result<String> {
    validate(&edited)?;
    let old_cells: HashMap<&str, &Value> = original["cells"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|cell| Some((cell["id"].as_str()?, cell)))
        .collect();
    let mut result = original.clone();
    if !result.is_object() {
        result = serde_json::json!({});
    }
    let mut cells = Vec::new();
    for cell in edited["cells"].as_array().context("Missing cells")? {
        let id = cell["id"].as_str().context("Missing cell ID")?;
        let old = old_cells.get(id).copied();
        let mut merged = old.cloned().unwrap_or_else(|| serde_json::json!({}));
        let mut edits = cell.clone();
        // Attachments have no editing UI. Never discard them during source edits.
        if edits["attachments"].is_null() {
            edits.as_object_mut().unwrap().remove("attachments");
        }
        merge(&mut merged, &edits);
        if preserve_outputs.contains(id)
            && let Some(old) = old
        {
            if let Some(outputs) = old.get("outputs") {
                merged["outputs"] = outputs.clone();
            }
            if let Some(count) = old.get("execution_count") {
                merged["execution_count"] = count.clone();
            }
        }
        cells.push(merged);
    }
    let mut edits = edited;
    edits.as_object_mut().unwrap().remove("cells");
    merge(&mut result, &edits);
    result["cells"] = Value::Array(cells);
    validate(&result)?;
    let serialized = serde_json::to_string_pretty(&result)? + "\n";
    let reparsed: Value = serde_json::from_str(&serialized)?;
    ensure!(
        reparsed == result,
        "Notebook serialization changed its contents"
    );
    Ok(serialized)
}

fn merge(original: &mut Value, edited: &Value) {
    match (original, edited) {
        (Value::Object(original), Value::Object(edited)) => {
            for (key, value) in edited {
                if let Some(old) = original.get_mut(key) {
                    merge(old, value);
                } else {
                    original.insert(key.clone(), value.clone());
                }
            }
        }
        (original, edited) => *original = edited.clone(),
    }
}

#[cfg(test)]
mod recovery_tests {
    use super::*;
    fn notebook(source: &str) -> String {
        serde_json::json!({"nbformat":4,"nbformat_minor":5,"metadata":{"custom":"preserved"},"cells":[{
            "id":"code", "cell_type":"code", "metadata":{}, "execution_count":null,
            "source":[source], "outputs":[{"output_type":"display_data","data":{"image/png":"abcd"},"metadata":{}}]
        }]}).to_string()
    }
    #[test]
    fn recovery_preserves_draft_outputs_and_unknown_metadata() {
        let original = notebook("before");
        let draft = notebook("after");
        let record = recovery_record(&original, &draft).unwrap();
        assert_eq!(restore_recovery(&record, &original).unwrap(), Some(draft));
    }
    #[test]
    fn recovery_refuses_independent_disk_edits() {
        let record = recovery_record(&notebook("before"), &notebook("draft")).unwrap();
        assert!(restore_recovery(&record, &notebook("external edit")).is_err());
    }
    #[test]
    fn recovery_ignores_a_draft_that_has_already_been_saved() {
        let draft = notebook("after");
        let record = recovery_record(&notebook("before"), &draft).unwrap();
        assert_eq!(restore_recovery(&record, &draft).unwrap(), None);
    }
    #[test]
    fn recovery_rejects_invalid_records() {
        assert!(restore_recovery("{}", &notebook("before")).is_err());
        assert!(recovery_record(&notebook("before"), "{\"cells\":[{}]}").is_err());
    }
}

/// Check structural requirements before allowing the editor to overwrite a file.
pub fn validate(notebook: &Value) -> Result<()> {
    ensure!(
        notebook["nbformat"] == 4,
        "Only notebook format 4 can be saved"
    );
    ensure!(
        notebook["nbformat_minor"].as_u64().is_some(),
        "Invalid notebook minor version"
    );
    ensure!(
        notebook["metadata"].is_object(),
        "Notebook metadata must be an object"
    );
    let mut ids = HashSet::new();
    for cell in notebook["cells"]
        .as_array()
        .context("Notebook cells must be an array")?
    {
        let id = cell["id"].as_str().context("Cell ID must be a string")?;
        ensure!(
            !id.is_empty()
                && id.len() <= 64
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
            "Invalid cell ID: {id}"
        );
        ensure!(ids.insert(id), "Duplicate cell ID: {id}");
        ensure!(
            cell["metadata"].is_object(),
            "Cell metadata must be an object"
        );
        ensure!(
            cell["source"].is_string()
                || cell["source"]
                    .as_array()
                    .is_some_and(|source| source.iter().all(Value::is_string)),
            "Invalid cell source"
        );
        match cell["cell_type"].as_str() {
            Some("code") => {
                ensure!(
                    cell["outputs"].is_array(),
                    "Code cell outputs must be an array"
                );
                ensure!(
                    cell["execution_count"].is_null() || cell["execution_count"].as_u64().is_some(),
                    "Invalid execution count"
                );
            }
            Some("markdown") | Some("raw") => {}
            _ => bail!("Unknown cell type"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn document() -> Value {
        json!({"nbformat":4,"nbformat_minor":5,"metadata":{"custom":{"keep":true}},"custom_top":42,"cells":[
            {"id":"code","cell_type":"code","metadata":{"tags":["important"]},"source":["x = 1"],"execution_count":3,"outputs":[{"output_type":"display_data","data":{"application/x.custom":{"rich":true},"text/plain":["one"]},"metadata":{"width":20}}],"custom_cell":7},
            {"id":"md","cell_type":"markdown","metadata":{},"source":["![image](attachment:pic.png)"],"attachments":{"pic.png":{"image/png":"base64"}}}
        ]})
    }
    #[test]
    fn editing_preserves_rich_outputs_attachments_and_unknown_fields() {
        let original = document();
        let mut edited = original.clone();
        edited["cells"][0]["source"] = json!(["x = 2"]);
        edited["cells"][0]["outputs"] = json!([]);
        edited["cells"][0]["execution_count"] = Value::Null;
        edited["cells"][1]["attachments"] = Value::Null;
        edited.as_object_mut().unwrap().remove("custom_top");
        edited["metadata"] = json!({});
        let saved: Value = serde_json::from_str(
            &prepare_save(&original, edited, &HashSet::from(["code".into()])).unwrap(),
        )
        .unwrap();
        assert_eq!(saved["cells"][0]["source"], json!(["x = 2"]));
        assert_eq!(
            saved["cells"][0]["outputs"],
            original["cells"][0]["outputs"]
        );
        assert_eq!(saved["cells"][0]["execution_count"], 3);
        assert_eq!(
            saved["cells"][1]["attachments"],
            original["cells"][1]["attachments"]
        );
        assert_eq!(saved["metadata"], original["metadata"]);
        assert_eq!(saved["custom_top"], 42);
    }
    #[test]
    fn clear_outputs_is_saved_and_does_not_restore_old_results() {
        let original = document();
        let mut edited = original.clone();
        edited["cells"][0]["outputs"] = json!([]);
        let saved: Value =
            serde_json::from_str(&prepare_save(&original, edited, &HashSet::new()).unwrap())
                .unwrap();
        assert_eq!(saved["cells"][0]["outputs"], json!([]));
    }
    #[test]
    fn reorder_and_delete_follow_ids_instead_of_cell_positions() {
        let original = document();
        let mut edited = original.clone();
        edited["cells"] = json!([original["cells"][1].clone()]);
        let saved: Value =
            serde_json::from_str(&prepare_save(&original, edited, &HashSet::new()).unwrap())
                .unwrap();
        assert_eq!(saved["cells"].as_array().unwrap().len(), 1);
        assert_eq!(saved["cells"][0]["id"], "md");
        assert_eq!(
            saved["cells"][0]["attachments"],
            original["cells"][1]["attachments"]
        );
    }
    #[test]
    fn malformed_or_duplicate_cells_are_refused() {
        let original = document();
        for (field, value) in [
            ("id", json!("md")),
            ("source", json!(123)),
            ("outputs", json!({})),
            ("cell_type", json!("unknown")),
        ] {
            let mut edited = original.clone();
            edited["cells"][0][field] = value;
            assert!(prepare_save(&original, edited, &HashSet::new()).is_err());
        }
    }
    #[test]
    fn no_trailing_newline_is_introduced_into_cell_source() {
        let original = document();
        let saved: Value = serde_json::from_str(
            &prepare_save(&original, original.clone(), &HashSet::new()).unwrap(),
        )
        .unwrap();
        assert_eq!(saved["cells"][0]["source"], json!(["x = 1"]));
    }
}
