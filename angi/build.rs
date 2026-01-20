use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../server/src");
    println!("cargo:rerun-if-changed=../server/Cargo.toml");
    println!("cargo:rerun-if-changed=../vm/src");
    println!("cargo:rerun-if-changed=../vm/Cargo.toml");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace = manifest_dir.parent().unwrap();

    let profile = env::var("PROFILE").unwrap(); // debug / release
    let target = env::var("TARGET").ok();

    let target_dir = workspace.join("server_target");

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("-p")
        .arg("server")
        .arg("--target-dir")
        .arg(&target_dir);

    if profile == "release" {
        cmd.arg("--release");
    }

    if let Some(t) = &target {
        cmd.arg("--target").arg(t);
    }

    let status = cmd.status().expect("failed to invoke cargo");

    assert!(status.success(), "server build failed");

    let bin_name = if cfg!(windows) { "server.exe" } else { "server" };

    let bin_path = match (&target, profile.as_str()) {
        (Some(t), p) => target_dir.join(t).join(p).join(bin_name),
        (None, p) => target_dir.join(p).join(bin_name),
    };

    assert!(bin_path.exists(), "server binary not found");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst = out_dir.join(bin_name);

    fs::copy(&bin_path, &dst).expect("failed to copy server");

    println!("cargo:rustc-env=SERVER_PATH={}", dst.display());
}
