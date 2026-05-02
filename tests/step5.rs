use cc_json_parser::handle_file;
use std::fs;

#[test]
fn json_checker_tests() {
    let dir = "test-data/json-checker-tests";
    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("json-checker-tests directory not found")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut failures: Vec<String> = vec![];

    for entry in entries {
        let path = entry.path();
        let filename = entry.file_name();
        let name = filename.to_string_lossy();
        let result = handle_file(path.to_str().unwrap());

        let passed = if name.starts_with("pass") {
            result.is_ok()
        } else {
            result.is_err()
        };

        if !passed {
            failures.push(format!(
                "{}: expected {}, got {}",
                name,
                if name.starts_with("pass") { "ok" } else { "err" },
                if result.is_ok() { "ok" } else { "err" }
            ));
        }
    }

    assert!(failures.is_empty(), "json-checker failures:\n{}", failures.join("\n"));
}
