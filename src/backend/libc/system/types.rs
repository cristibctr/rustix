use crate::backend::c;

/// `sysinfo`
#[cfg(linux_kernel)]
pub type Sysinfo = c::sysinfo;

#[cfg(not(all(target_os = "wasi", not(target_vendor = "wasmer"))))]
pub(crate) type RawUname = c::utsname;
