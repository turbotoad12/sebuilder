use std::io;
use std::process::{Command, Stdio};

pub fn run_cross_platform(cmd: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    let mut command = Command::new("cmd");
    #[cfg(target_os = "windows")]
    command.arg("/C");

    #[cfg(not(target_os = "windows"))]
    let mut command = Command::new("sh");
    #[cfg(not(target_os = "windows"))]
    command.arg("-c");

    command
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}
