use crate::backend::c;

/// `sysinfo`
#[cfg(linux_kernel)]
pub type Sysinfo = c::sysinfo;

#[cfg(not(all(target_os = "wasi", target_env = "p2")))]
pub(crate) type RawUname = c::utsname;
