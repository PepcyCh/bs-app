use std::process::Command;

fn rollup() {
    if cfg!(target_os = "windows") {
        return;
    }

    println!("cargo:rerun-if-changed=js/src/*");
    println!("cargo:rerun-if-changed=js/rollup.config.js");

    let status = Command::new("rollup")
        .args(&["-c"])
        .current_dir("./js")
        .status()
        .expect("Failed to run rollup");
    if !status.success() {
        panic!("Failed to bundle js. Process exits with {}", status);
    }
    eprintln!("Js is bundled");
}

fn main() {
    rollup();
}
