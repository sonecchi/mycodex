use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    let prompt_md = PathBuf::from(&manifest_dir).join("prompt.md");
    let prompt_local_md = PathBuf::from(&manifest_dir).join("prompt.local.md");
    let out_path = PathBuf::from(&out_dir).join("base_instructions.txt");

    // Always watch both files; if either changes, rebuild.
    println!("cargo:rerun-if-changed={}", prompt_md.display());
    println!("cargo:rerun-if-changed={}", prompt_local_md.display());

    let (source_path, human): (PathBuf, String) = if prompt_local_md.is_file() {
        (
            prompt_local_md.clone(),
            format!("prompt: {}", prompt_local_md.display()),
        )
    } else {
        (
            prompt_md.clone(),
            "prompt: codex-rs/core/prompt.md".to_string(),
        )
    };

    // Copy the chosen prompt into OUT_DIR for include_str! at compile time.
    fs::copy(&source_path, &out_path).expect("failed to write base_instructions.txt");

    // Surface a stable human-readable string for the TUI banner.
    println!(
        "cargo:rustc-env=CODEX_BASE_INSTRUCTIONS_HUMAN={}",
        human
    );
}

