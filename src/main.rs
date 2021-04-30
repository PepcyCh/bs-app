use std::process::Command;

fn start_server() {
    let exe_name = if cfg!(target_os = "windows") {
        "./out/bs-backend.exe"
    } else {
        "./out/bs-backend"
    };
    Command::new(exe_name)
        .current_dir("./out")
        .status()
        .expect("Fialed to start server");
}

fn main() {
    start_server();
}
