#![allow(dead_code)]
use anyhow::{Context, Result};
use clap::crate_version;
use lazy_static::lazy_static;
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

extern crate os_type;

// We need to specify our version in a static because we've painted clap
// into a corner. We've told it that every string we give it will be
// 'static, but we need to build the version string dynamically. We can
// fake the 'static lifetime with lazy_static.
lazy_static! {
    pub static ref LONG_VERSION: String = long_version(None, true);
}

pub fn path_readable_file(value: &OsStr) -> Result<(), String> {
    let path = PathBuf::from(value); //.as_ref();

    if path.is_dir() {
        return Err(format!(
            "{}: Input path must be a file, not a directory",
            path.display()
        ));
    }

    File::open(&path)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", path.display(), e))
}

pub fn generate_service_agent() {
    let os = os_type::current_platform();
    eprintln!("Detected OS: {:?}", os.os_type);
}

/// Return the "long" format of ripgrep's version string.
///
/// If a revision hash is given, then it is used. If one isn't given, then
/// the HERMESD_BUILD_GIT_HASH env var is inspected for it. If that isn't set,
/// then a revision hash is not included in the version string returned.
///
/// If `cpu` is true, then the version string will include the compiled and
/// runtime CPU features.
fn long_version(revision_hash: Option<&str>, cpu: bool) -> String {
    // Do we have a git hash?
    // (Yes, if ripgrep was built on a machine with `git` installed.)
    let hash = match revision_hash.or(option_env!("HERMESD_BUILD_GIT_HASH")) {
        None => String::new(),
        Some(githash) => format!(" (rev {})", githash),
    };
    if !cpu {
        format!("{}{}", crate_version!(), hash,)
    } else {
        let runtime = runtime_cpu_features();
        if runtime.is_empty() {
            format!(
                "{}{}\n{} (compiled)",
                crate_version!(),
                hash,
                compile_cpu_features().join(" ")
            )
        } else {
            format!(
                "{}{}\n{} (compiled)\n{} (runtime)",
                crate_version!(),
                hash,
                compile_cpu_features().join(" "),
                runtime.join(" ")
            )
        }
    }
}

/// Returns the relevant CPU features enabled at compile time.
fn compile_cpu_features() -> Vec<&'static str> {
    let mut features = vec![];
    if cfg!(feature = "simd-accel") {
        features.push("+SIMD");
    } else {
        features.push("-SIMD");
    }
    if cfg!(feature = "avx-accel") {
        features.push("+AVX");
    } else {
        features.push("-AVX");
    }
    features
}

/// Returns the relevant CPU features enabled at runtime.
#[cfg(target_arch = "x86_64")]
fn runtime_cpu_features() -> Vec<&'static str> {
    // This is kind of a dirty violation of abstraction, since it assumes
    // knowledge about what specific SIMD features are being used.

    let mut features = vec![];
    if is_x86_feature_detected!("ssse3") {
        features.push("+SIMD");
    } else {
        features.push("-SIMD");
    }
    if is_x86_feature_detected!("avx2") {
        features.push("+AVX");
    } else {
        features.push("-AVX");
    }
    features
}

/// Returns the relevant CPU features enabled at runtime.
#[cfg(not(target_arch = "x86_64"))]
fn runtime_cpu_features() -> Vec<&'static str> {
    vec![]
}

pub fn prompt_confirm_with_default(message: &str, default: bool) -> Result<bool> {
    let confirm = inquire::Confirm::new(message);
    confirm
        .with_default(default)
        // .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for confirm")
}