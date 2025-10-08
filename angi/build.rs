use std::{env, fs, path::PathBuf, process::Command };

fn main() {
   let workspace = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();

    let status = Command::new("cargo")
        .args(["build", "-p", "server", "--release", "--target-dir", "../server_target"])
        .status()
        .expect("Fail to build");

    assert!(status.success());

    let runtime_path = workspace.join("server_target/release/server");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join("server");

    fs::copy(&runtime_path, &dest_path).expect("Fail to copy");

    println!("cargo:rustc-env=RUNTIME_PATH={}", dest_path.display());
}
