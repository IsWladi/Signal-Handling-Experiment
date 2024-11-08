use windows::Win32::System::Console::{SetConsoleCtrlHandler, CTRL_C_EVENT, CTRL_CLOSE_EVENT};
use windows::Win32::Foundation::BOOL;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

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
        SetConsoleCtrlHandler(Some(console_handler), BOOL(1)).expect("Failed to set control handler");
    }

    // Main application code
    while !SHOULD_STOP.load(Ordering::SeqCst) {
        println!("Running...");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Graceful shutdown completed.");
}
