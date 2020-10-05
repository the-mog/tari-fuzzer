// Copyright 2016 rust-fuzz developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

// Copyright 2020 Tari Project. Licensed under Apache-2.0.

//! Command line utility to run fuzz tests.
//!
//! Adopted from https://github.com/rust-fuzz/targets
//! Further adopted from https://github.com/tikv/tikv/tree/master/fuzz

use std::{
    env,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{corpus::gen_corpus, engines::Fuzzer};
use anyhow::{anyhow, Context, Result};
use cargo_metadata::MetadataCommand;
use lazy_static::lazy_static;
use log::{info, trace};

lazy_static! {
    static ref WORKSPACE_ROOT: PathBuf = MetadataCommand::new().no_deps().exec().unwrap().workspace_root;
}

/// Write the fuzz target source file from corresponding template file.
///
/// `target` must be a valid target.
fn write_fuzz_target_source_file(fuzzer: &Fuzzer, target: &str) -> Result<()> {
    trace!("Writing target fuzz test to file");
    let template_file_path = fuzzer.directory().join("template.rs");
    let template = fs::read_to_string(&template_file_path)
        .context(format!("Error reading template file {}", template_file_path.display()))?;

    let target_file_path = fuzzer.directory().join(&format!("src/bin/{}.rs", target));
    info!("Target file written to {:?}", &target_file_path);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&target_file_path)
        .context(format!(
            "Error writing fuzz target source file {}",
            target_file_path.display()
        ))?;
    let source = template
        .replace("__FUZZ_CLI_TARGET__", &target)
        .replace("__FUZZ_GENERATE_COMMENT__", "NOTE: AUTO GENERATED FROM `template.rs`");
    file.write_all(source.as_bytes())?;
    Ok(())
}

fn create_artifact_dir(base: impl AsRef<Path>, target: &str) -> Result<PathBuf> {
    trace!("Creating artifact directory for fuzzer output");
    let base = base.as_ref();
    let artifact_dir = base.join(&format!("artifact-{}", target));
    fs::create_dir_all(&artifact_dir).context(format!(
        "unable to create corpus dir for {}{}",
        base.display(),
        target
    ))?;
    info!("Artifact directory created: {:?}", &artifact_dir);
    Ok(artifact_dir)
}

fn pre_check(command: &mut Command, hint: &str) -> Result<()> {
    let check = command.stdout(Stdio::null()).stderr(Stdio::null()).status().unwrap();
    if !check.success() {
        Err(anyhow!(
            "Pre-checking for fuzzing failed. Consider run `{}` before fuzzing.",
            hint
        ))
    } else {
        Ok(())
    }
}

/// Run one target fuzz test using AFL
pub fn run_afl(target: &str, fuzzer: Fuzzer, seeds: usize) -> Result<()> {
    trace!("Getting ready to start fuzzing with AFL");
    write_fuzz_target_source_file(&fuzzer, &target)?;
    let artifact_dir = create_artifact_dir(fuzzer.directory(), target)?;
    let corpus_dir = gen_corpus(&target, fuzzer, seeds)?;

    pre_check(Command::new("cargo").args(&["afl", "--version"]), "cargo install afl")?;

    // 1. cargo afl build (in fuzzer-afl directory)
    let fuzzer_build = Command::new("cargo")
        .args(&["afl", "build", "--bin", target])
        .current_dir(fuzzer.directory())
        .spawn()
        .context(format!("Failed to build {}", fuzzer))?
        .wait()
        .context(format!("Failed to complete building {}", fuzzer))?;

    if !fuzzer_build.success() {
        return Err(anyhow!(
            "error building afl instrumented binary, exit code {:?}",
            fuzzer_build.code()
        ));
    }

    // 2. cargo afl fuzz -i {seed_dir} -o {corpus_dir} target/debug/{instrumented_binary}
    let instrumented_bin = WORKSPACE_ROOT.join("target/debug").join(target);
    let fuzzer_bin = Command::new("cargo")
        .args(&["afl", "fuzz"])
        .arg("-i")
        .arg(&corpus_dir)
        .arg("-o")
        .arg(&artifact_dir)
        .arg(&instrumented_bin)
        .current_dir(fuzzer.directory())
        .spawn()
        .context(format!("Failed to run {}", fuzzer))?
        .wait()
        .context(format!("Failed to wait {}", fuzzer))?;

    if !fuzzer_bin.success() {
        return Err(anyhow!("{} exited with code {:?}", fuzzer, fuzzer_bin.code()));
    }

    Ok(())
}

/// Run one target fuzz test using Honggfuzz
pub fn run_honggfuzz(target: &str, fuzzer: Fuzzer, seeds: usize) -> Result<()> {
    trace!("Getting ready to start fuzzing with Honggfuzz");
    write_fuzz_target_source_file(&fuzzer, &target)?;
    let _artifact_dir = create_artifact_dir(fuzzer.directory(), target)?;
    let corpus_dir = gen_corpus(&target, fuzzer, seeds)?;
    pre_check(
        Command::new("cargo").args(&["hfuzz", "version"]),
        "cargo install honggfuzz --version 0.5.45",
    )?;

    let mut rust_flags = env::var("RUSTFLAGS").unwrap_or_default();
    rust_flags.push_str(" -Z sanitizer=address");

    let hfuzz_args = format!(
        "-f {} --exit_upon_crash {}",
        corpus_dir.to_string_lossy(),
        env::var("HFUZZ_RUN_ARGS").unwrap_or_default()
    );

    let fuzzer_bin = Command::new("cargo")
        .args(&["hfuzz", "run", target])
        .env("RUSTFLAGS", &rust_flags)
        .env("HFUZZ_RUN_ARGS", &hfuzz_args)
        .current_dir(fuzzer.directory())
        .spawn()
        .context(format!("Failed to run {}", fuzzer))?
        .wait()
        .context(format!("Failed to wait {}", fuzzer))?;

    if !fuzzer_bin.success() {
        return Err(anyhow!("{} exited with code {:?}", fuzzer, fuzzer_bin.code()));
    }

    Ok(())
}

/// Run one target fuzz test using Libfuzzer
pub fn run_libfuzzer(target: &str, fuzzer: Fuzzer, seeds: usize) -> Result<()> {
    trace!("Getting ready to start fuzzing with Libfuzzer");
    write_fuzz_target_source_file(&fuzzer, &target)?;
    let artifact_dir = create_artifact_dir(fuzzer.directory(), target)?;
    let corpus_dir = gen_corpus(&target, fuzzer, seeds)?;
    //info!("Corpus generated!");

    #[cfg(target_os = "macos")]
    let target_platform = "x86_64-apple-darwin";
    #[cfg(target_os = "linux")]
    let target_platform = "x86_64-unknown-linux-gnu";
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    panic!("libfuzzer-sys only supports Linux and macOS");

    // FIXME: The -C codegen-units=1 and -C incremental=..
    // below seem to workaround some difficult issues in Rust nightly
    // https://github.com/rust-lang/rust/issues/53945.
    // If this is ever fixed remember to remove the fuzz-incremental
    // entry from .gitignore.
    let mut rust_flags = env::var("RUSTFLAGS").unwrap_or_default();
    rust_flags.push_str(
        "--cfg fuzzing -C codegen-units=1 -C incremental=fuzz-incremental -C passes=sancov -C \
         llvm-args=-sanitizer-coverage-level=4 -C llvm-args=-sanitizer-coverage-trace-compares -C \
         llvm-args=-sanitizer-coverage-inline-8bit-counters -C llvm-args=-sanitizer-coverage-trace-geps -C \
         llvm-args=-sanitizer-coverage-prune-blocks=0 -C debug-assertions=on -C debuginfo=0 -C opt-level=3 -Z \
         sanitizer=address",
    );

    let mut asan_options = env::var("ASAN_OPTIONS").unwrap_or_default();
    asan_options.push_str(" detect_odr_violation=0");

    let fuzzer_bin = Command::new("cargo")
        .args(&["run", "--target", &target_platform, "--bin", target, "--"])
        .arg(&corpus_dir)
        .arg(&artifact_dir)
        .env("RUSTFLAGS", &rust_flags)
        .env("ASAN_OPTIONS", &asan_options)
        .current_dir(fuzzer.directory())
        .spawn()
        .context(format!("Failed to run {}", fuzzer))?
        .wait()
        .context(format!("Failed to wait {}", fuzzer))?;

    if !fuzzer_bin.success() {
        return Err(anyhow!("{} exited with code {:?}", fuzzer, fuzzer_bin.code()));
    }

    Ok(())
}
