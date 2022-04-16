use tempfile::TempDir;
use std::process::Command;

fn output(cmd: &mut Command) {
    println!("{cmd:?}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !stdout.is_empty() {
        println!("{stdout}");
    }
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !stderr.is_empty() {
        println!("{stderr}");
    }
}

fn main() {
    let td = TempDir::new().unwrap();
    println!("{:?}", td.path());
    output(Command::new("ls").arg("-al").arg(td.path()));
    output(Command::new("who").arg("am").arg("i"));
}
