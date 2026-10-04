//! Headless serialization/recovery benchmark. GUI measurements are separate.
use std::{collections::HashSet, hint::black_box, time::Instant};

fn main() -> anyhow::Result<()> {
    let mut results = Vec::new();
    for count in [50, 500, 2_000] {
        let cells: Vec<_> = (0..count)
            .map(|i| {
                serde_json::json!({
                    "id": format!("cell-{i}"), "cell_type": "code", "metadata": {"custom": i},
                    "source": ["values = [x * x for x in range(100)]\n", "values[:10]\n"],
                    "execution_count": i, "outputs": [{"output_type": "display_data",
                        "metadata": {"custom": true}, "data": {"text/plain": ["[0, 1, 4, 9]"],
                            "text/html": ["<table><tr><td>0</td><td>1</td></tr></table>"]}}]
                })
            })
            .collect();
        let original = serde_json::json!({"nbformat": 4, "nbformat_minor": 5,
            "metadata": {"custom": {"retained": true}}, "cells": cells});
        let original_text = serde_json::to_string(&original)?;
        let preserve: HashSet<String> = (0..count).map(|i| format!("cell-{i}")).collect();
        let mut edited = original.clone();
        edited["cells"][0]["source"] = serde_json::json!(["print('edited')\n"]);
        // Warm allocator and parsing paths; report median of seven samples.
        notebook_safety::prepare_save(&original, edited.clone(), &preserve)?;
        let mut save = Vec::new();
        let mut recovery = Vec::new();
        for _ in 0..7 {
            let started = Instant::now();
            let draft =
                notebook_safety::prepare_save(black_box(&original), edited.clone(), &preserve)?;
            save.push(started.elapsed().as_secs_f64() * 1000.);
            let started = Instant::now();
            let record = notebook_safety::recovery_record(&original_text, &draft)?;
            let restored = notebook_safety::restore_recovery(&record, &original_text)?.unwrap();
            assert_eq!(draft, restored);
            recovery.push(started.elapsed().as_secs_f64() * 1000.);
        }
        save.sort_by(f64::total_cmp);
        recovery.sort_by(f64::total_cmp);
        results.push(
            serde_json::json!({"cells": count, "input_bytes": original_text.len(),
            "save_median_ms": save[3], "recovery_roundtrip_median_ms": recovery[3]}),
        );
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "scope": "headless notebook serialization and recovery; excludes GUI and kernel",
            "samples": 7, "os": std::env::consts::OS, "architecture": std::env::consts::ARCH,
            "results": results
        }))?
    );
    Ok(())
}
