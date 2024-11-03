use std::io::Error;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
                    if libc::setsid() == -1 {
                        // create new session detached from the parent process
                        return Err(std::io::Error::last_os_error());
                    }

                    libc::signal(libc::SIGINT, libc::SIG_IGN); // ignore SIGINT (only for the child process)

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
