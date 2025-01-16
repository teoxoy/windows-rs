#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtDeviceIoControlFile(filehandle: super::super::super::Win32::Foundation::HANDLE, event: Option<super::super::super::Win32::Foundation::HANDLE>, apcroutine: Option<super::super::super::Win32::System::IO::PIO_APC_ROUTINE>, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_link::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtDeviceIoControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine.unwrap_or(core::mem::zeroed()) as _, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, iocontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
