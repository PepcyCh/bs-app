use std::process::Command;

fn main() {
    // TODO - there is something wrong on Windows...
    if cfg!(target_os = "windows") {
        return;
    }

    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir("./js")
        .status()
        .expect("Failed to run npm build");
    if !status.success() {
        panic!("Failed to bundle js. Process exits with {}", status);
    }
    eprintln!("JS is bundled");
}
