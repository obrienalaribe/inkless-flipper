#![no_std]

// Entry points ("exported functions"): The only requirement by pallet contracts is
// to expose those two functions with exactly this name and signature.
// ----------------------------------------------------------------------------

/// Called when a contract is instantiated (happens only once in its lifetime)
#[no_mangle]
pub fn deploy() {
    // TODO: initialize storage
    api::print("contract deployed");
}

/// Called every time you call a contract.
#[no_mangle]
pub fn call() {
    let mut buffer = [0u8; 32];
    let input = api::input(&mut buffer);

    match input.get(0) {
        Some(0) => {
            api::print("flip() called");
            // TODO: implement flip() function
        }
        Some(1) => {
            api::print("get() called");
            // TODO: implement get() function
        }
        _ => {
            api::print("unknown function called");
            // For two reasons this will never be printed:
            // 1. Our panic handler doesn't do anything with this message
            // 2. We compiled core with `panic_immediate_abort` which skips the message formatting
            //		This means our panic handler doens't even receive this message.
            panic!("this will never be printed");
        }
    }
}

// ----------------------------------------------------------------------------

///	Required by rust when not using std. Defines what happens on panic.
/// std implements sophisticated logic like stack unwinding here which is
/// operating system specific. We just stop the execution. For example, we could
/// log a message to pallet-contracts before dying here.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // This is what we call a "trap" in wasm (stops the execution)
    core::arch::wasm32::unreachable()
}

/// Wraps the unsafe imported functions for easy api access.
/// A contract does not need to import any functions but it would be a pretty useless
/// program without doing so.
#[allow(dead_code)]
mod api {
    /// Messages can be seen when calling a contract via the `Developer -> Runtime` tab.
    pub fn print(message: &str) {
        unsafe {
            sys::debug_message(message.as_ptr(), message.len() as u32);
        }
    }

    pub fn seal_return(data: &[u8]) {
        unsafe {
            sys::seal_return(0, data.as_ptr(), data.len() as u32);
        }
    }

    pub fn input(mut out_buffer: &mut [u8]) -> &mut [u8] {
        let mut len = out_buffer.len() as u32;
        unsafe {
            sys::input(out_buffer.as_mut_ptr(), &mut len as *mut u32);
        }
        shrink_slice(&mut out_buffer, len as usize);
        out_buffer
    }

    pub fn get_storage<'a>(key: &[u8], mut out_buffer: &'a mut [u8]) -> &'a mut [u8] {
        let mut len = out_buffer.len() as u32;
        unsafe {
            sys::get_storage(
                key.as_ptr(),
                key.len() as u32,
                out_buffer.as_mut_ptr(),
                &mut len as *mut u32,
            );
        }
        shrink_slice(&mut out_buffer, len as usize);
        out_buffer
    }

    pub fn set_storage(key: &[u8], value: &[u8]) {
        unsafe {
            sys::set_storage(
                key.as_ptr(),
                key.len() as u32,
                value.as_ptr(),
                value.len() as u32,
            );
        }
    }

    fn shrink_slice(output: &mut &mut [u8], new_len: usize) {
        let tmp = core::mem::take(output);
        *output = &mut tmp[..new_len];
    }

    /// Imported functions
    /// Your only way to communicate with the outside world.
    /// Check https://docs.rs/pallet-contracts/latest/pallet_contracts/api_doc/index.html
    /// for a documentation of this API.
    ///
    /// Don't use them directly. We implemented wrappers for you.
    mod sys {
        #[link(wasm_import_module = "seal0")]
        extern "C" {
            /// Print a debug message (only visible when dry-running)
            pub fn debug_message(str_ptr: *const u8, str_len: u32) -> u32;
            /// Return data to the caller
            pub fn seal_return(flags: u32, data_ptr: *const u8, data_len: u32) -> !;
            /// Read arguments to this contract call/instantiate
            pub fn input(buf_ptr: *mut u8, buf_len_ptr: *mut u32);
        }

        #[link(wasm_import_module = "seal1")]
        extern "C" {
            /// Read bytes from storage
            pub fn get_storage(
                key_ptr: *const u8,
                key_len: u32,
                out_ptr: *mut u8,
                out_len_ptr: *mut u32,
            ) -> u32;
        }

        #[link(wasm_import_module = "seal2")]
        extern "C" {
            /// Write bytes to storage
            pub fn set_storage(
                key_ptr: *const u8,
                key_len: u32,
                value_ptr: *const u8,
                value_len: u32,
            ) -> u32;
        }
    }
}
