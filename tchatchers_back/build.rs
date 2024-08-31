use std::process::Command;

fn main() {
    if let Ok(out) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(out.stdout) {
            println!("cargo:rustc-env=GIT_REV={}", stdout);
        }
    }
}
