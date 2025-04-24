use std::process::Command;

pub fn m5_fail() {
    Command::new("/usr/local/bin/m5")
        .arg("fail")
        .arg("4")
        .spawn()
        .inspect_err(|e| println!("failed to execute m5 magic instr: {e}"))
        .ok();
}
