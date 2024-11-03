use std::io::Error;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Execute a command in a infinite loop until SIGINT is received.
/// The SIGINT signal have to be ignored in the child process to avoid the abrupt termination of executed command.
fn main() -> Result<(), Error> {
    // signal handling with crate: signal-hook
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
    let mut i = 0;

    // execute command while the term flag is false (SIGINT is not received)
    while !term.load(Ordering::Relaxed) {
        let child = unsafe {
            Command::new("sh")
                .arg("-c")
                .arg(format!("sleep 2 && echo -n 'hello #{} after 2 seconds'", i))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .pre_exec(|| {
                    // create new session detached from the parent process
                    if libc::setsid() == -1 {
                        return Err(std::io::Error::last_os_error());
                    }

                    Ok(())
                })
                .spawn()
                .expect("failed to execute command")
        };

        let output = child.wait_with_output().expect("failed to wait on child");
        let stdout = output.stdout;
        let stderr = output.stderr;
        println!(
            "stdout: {}, stderr: {}",
            String::from_utf8(stdout).unwrap(),
            String::from_utf8(stderr).unwrap()
        );
        i += 1;
    }
    Ok(())
}
