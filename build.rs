use anyhow::{bail, Result};
use std::process::Command;

fn build_frontend() -> Result<()> {
    eprintln!("Building frontend...");

    let status = Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-name",
            "wasm",
            "--out-dir",
            "../out",
        ])
        .current_dir("./frontend")
        .status()
        .expect("Failed to build frontend");
    eprintln!("Frontend is built");

    let mut frontend_files = [
        glob::glob("./frontend/static/*.html")?,
        glob::glob("./frontend/static/*.css")?,
    ];
    let frontend_files: Vec<_> = frontend_files
        .iter_mut()
        .flatten()
        .map(|glob_result| glob_result.unwrap())
        .collect();

    let mut copy_option = fs_extra::dir::CopyOptions::new();
    copy_option.overwrite = true;
    fs_extra::copy_items(&frontend_files, "./out", &copy_option)
        .expect("Failed to copy frontend outputs");

    if status.success() {
        Ok(())
    } else {
        bail!(format!(
            "Failed to build frontend. Process exits with {}",
            status
        ))
    }
}

fn build_backend() -> Result<()> {
    eprintln!("Building backend...");

    let args = if cfg!(debug_assertions) {
        vec!["build"]
    } else {
        vec!["build", "--release"]
    };
    let status = Command::new("cargo")
        .args(&args)
        .current_dir("./backend")
        .status()
        .expect("Failed to build backend");
    eprintln!("Backend is built");

    let mut backend_files = [glob::glob("./backend/config/*.toml")?];
    let backend_files: Vec<_> = backend_files
        .iter_mut()
        .flatten()
        .map(|glob_result| glob_result.unwrap())
        .collect();

    let mut copy_option = fs_extra::dir::CopyOptions::new();
    copy_option.overwrite = true;
    fs_extra::copy_items(&backend_files, "./out", &copy_option)
        .expect("Failed to copy backend outputs");

    let target_dir = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let exe_name = if cfg!(target_os = "windows") {
        "bs-backend.exe"
    } else {
        "bs-backend"
    };
    let exe_path = format!("./backend/target/{}/{}", target_dir, exe_name);

    let mut copy_option = fs_extra::dir::CopyOptions::new();
    copy_option.overwrite = true;
    fs_extra::copy_items(&[exe_path], "./out", &copy_option)
        .expect("Failed to copy backend outputs");

    if status.success() {
        Ok(())
    } else {
        bail!(format!(
            "Failed to build backend. Process exits with {}",
            status
        ))
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=common/*");
    println!("cargo:rerun-if-changed=frontend/*");
    println!("cargo:rerun-if-changed=backend/*");

    build_frontend()?;
    build_backend()?;
    eprintln!("Build is done");

    Ok(())
}
