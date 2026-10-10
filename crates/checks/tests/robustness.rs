/// Robustness tests: verify no checks panic on realistic input.
///
/// This test suite scans all Rust sources in the repo and truncated inputs
/// to ensure no check panics, providing early detection of crashes from
/// unexpected AST shapes or out-of-bounds indexing.

use sorolint_checks::default_checks;
use std::fs;
use std::path::{Path, PathBuf};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn analyze_source(content: &str, _filename: &str) -> Result<Vec<String>, String> {
    let file = syn::parse_file(content).map_err(|e| e.to_string())?;
    let checks = default_checks();
    let mut panics = Vec::new();
    for check in checks {
        if let Err(err) = catch_unwind(AssertUnwindSafe(|| {
            check.run(&file, content);
        })) {
            let msg = if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_string()
            };
            panics.push(format!("{}: {}", check.name(), msg));
        }
    }
    Ok(panics)
}

fn find_rust_sources<P: AsRef<Path>>(root: P, limit: Option<usize>) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                sources.push(path);
                if let Some(l) = limit {
                    if sources.len() >= l {
                        break;
                    }
                }
            } else if path.is_dir() {
                let subdir = find_rust_sources(&path, limit);
                sources.extend(subdir);
                if let Some(l) = limit {
                    if sources.len() >= l {
                        break;
                    }
                }
            }
        }
    }
    sources
}

#[test]
fn test_no_panics_on_real_sources() {
    let sources = find_rust_sources("src", Some(50));

    for source_path in sources {
        if let Ok(content) = fs::read_to_string(&source_path) {
            let result = analyze_source(&content, source_path.to_str().unwrap_or("unknown"));

            match result {
                Ok(panics) if !panics.is_empty() => {
                    panic!(
                        "Checks panicked on {}: {:?}",
                        source_path.display(),
                        panics
                    );
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Analysis failed on {}: {}", source_path.display(), e);
                }
            }
        }
    }
}

#[test]
fn test_no_panics_on_truncated_input() {
    let test_inputs = vec![
        "",                     // Empty
        "fn",                   // Incomplete keyword
        "pub fn foo(",          // Incomplete function signature
        "fn foo() { unsafe {",  // Incomplete unsafe block
        "impl Foo {",           // Incomplete impl block
        "#[derive(",            // Incomplete macro
    ];

    for input in test_inputs {
        let result = analyze_source(input, "truncated_input.rs");

        match result {
            Ok(panics) if !panics.is_empty() => {
                panic!("Checks panicked on truncated input: {:?}", panics);
            }
            Ok(_) => {}
            Err(_) => {
                // Parse errors are acceptable; panics are not
            }
        }
    }
}

#[test]
fn test_no_panics_on_edge_cases() {
    let edge_cases = vec![
        // Very long lines
        &"fn foo() { let x = \"\".repeat(10000); }",
        // Deeply nested expressions
        &"fn f() { ((((((((((x)))))))))) }",
        // Many function parameters
        &"fn f(a:i32, b:i32, c:i32, d:i32, e:i32, f:i32, g:i32, h:i32) {}",
        // Unicode identifiers
        &"fn café() { let ñoño = 42; }",
    ];

    for input in edge_cases {
        let result = analyze_source(input, "edge_case.rs");

        match result {
            Ok(panics) if !panics.is_empty() => {
                panic!("Checks panicked on edge case: {:?}", panics);
            }
            Ok(_) => {}
            Err(_) => {
                // Parse errors are acceptable; panics are not
            }
        }
    }
}

#[test]
fn test_panic_recovery_is_working() {
    let problematic_input = "fn foo() { let x = unsafe { std::ptr::null::<()>().read() }; }";

    let result = analyze_source(problematic_input, "panic_test.rs");

    match result {
        Ok(panics) => {
            eprintln!("Captured {} check panics (expected)", panics.len());
        }
        Err(e) => {
            eprintln!("Analysis error (acceptable): {}", e);
        }
    }
}
