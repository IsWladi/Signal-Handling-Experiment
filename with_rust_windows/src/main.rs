use winapi::um::wincon::{SetConsoleCtrlHandler, CTRL_CLOSE_EVENT, CTRL_C_EVENT};

unsafe extern "system" fn console_handler(ctrl_type: u32) -> i32 {
    match ctrl_type {
        CTRL_C_EVENT | CTRL_CLOSE_EVENT => {
            println!("Received console close event, shutting down...");
            // Perform cleanup here
            1 // Return TRUE to indicate that we handled the event
        }
        _ => 0, // Return FALSE for events we're not handling
    }
}

fn main() {
    unsafe {
        SetConsoleCtrlHandler(Some(console_handler), 1);
    }

    // Main application code
    loop {
        println!("Running...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
