use std::io::Command;
use std::io::fs::PathExtensions;
use std::os;

fn main() {
    let dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").expect("should run in cargo"));
    os::change_dir(&dir).unwrap();

    if Path::new("crates.io-index").exists() {
        return
    }

    Command::new("git")
        .arg("clone")
        .arg("--depth").arg("1")
        .arg("https://github.com/rust-lang/crates.io-index")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
