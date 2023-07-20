use std::os::raw::{c_int, c_void};

#[no_mangle]
#[cfg(windows)]
pub extern "system" fn DllMain(_handle: *mut c_void, reason: c_int, _reserved: *mut c_void) -> c_int {
    match reason {
        1 => {
            // DLL_PROCESS_ATTACH
            // Perform initialization tasks here
            println!("DLL_PROCESS_ATTACH");
        }
        0 => {
            // DLL_PROCESS_DETACH
            // Perform cleanup tasks here
            println!("DLL_PROCESS_DETACH");
        }
        _ => {}
    }
    1 // Return a non-zero value to indicate success
}