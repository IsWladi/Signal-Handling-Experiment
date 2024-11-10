use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Console::{SetConsoleCtrlHandler, CTRL_CLOSE_EVENT, CTRL_C_EVENT};

//
use std::os::windows::process::CommandExt;
//https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.WIN32_ERROR.html
use windows::Win32::Foundation::WIN32_ERROR;
//https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/constant.CREATE_NEW_CONSOLE.html
use windows::Win32::System::Threading::CREATE_NEW_CONSOLE;
//https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/constant.DETACHED_PROCESS.html
use windows::Win32::System::Threading::DETACHED_PROCESS;

static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn console_handler(ctrl_type: u32) -> BOOL {
    match ctrl_type {
        CTRL_C_EVENT | CTRL_CLOSE_EVENT => {
            println!("Received console close event, shutting down...");
            SHOULD_STOP.store(true, Ordering::SeqCst); // Signal the main loop to stop
            BOOL(1) // Return TRUE to indicate that we handled the event
        }
        _ => BOOL(0), // Return FALSE for events we're not handling
    }
}

fn main() {
    unsafe {
        SetConsoleCtrlHandler(Some(console_handler), BOOL(1))
            .expect("Failed to set control handler");
    }

    // Print process ID
    println!("Main pid: {}", std::process::id());

    // Main application code
    let mut i = 0;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    while !SHOULD_STOP.load(Ordering::SeqCst) && i < 10 {
        println!("Running {}...", i + 1);

        handles.push(thread::spawn(|| {
            exec(i);
        }));
    }

    println!("Waiting for child processes to complete...");
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Graceful shutdown completed.");
}

fn exec(i: i64) -> Result<(), WIN32_ERROR> {
    let child = CommandExt::new("cmd")
        .arg("/C")
        .arg("timeout /t 5 && echo 'Hello from the child process!'")
        .creation_flags(DETACHED_PROCESS | CREATE_NEW_CONSOLE)
        .spawn()
        .expect("the child process was not created");

    println!("Child pid: {}, execution #{}", child.id(), i);
    Ok(())
}
