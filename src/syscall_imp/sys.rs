use core::ffi::{c_char, c_int};
use alloc::{borrow::ToOwned, ffi::CString, sync::Arc};
use arceos_posix_api::{self as api, get_file_like, handle_file_path, File};
use axerrno::LinuxResult;
use axfs::api::OpenOptions;

use crate::{ctypes::{UTIME_NOW, UTIME_OMIT}, ptr::{PtrWrapper, UserConstPtr, UserPtr}};

pub fn sys_getuid() -> LinuxResult<isize> {
    Ok(0)
}

#[repr(C)]
pub struct UtsName {
    /// sysname
    pub sysname: [u8; 65],
    /// nodename
    pub nodename: [u8; 65],
    /// release
    pub release: [u8; 65],
    /// version
    pub version: [u8; 65],
    /// machine
    pub machine: [u8; 65],
    /// domainname
    pub domainname: [u8; 65],
}

impl Default for UtsName {
    fn default() -> Self {
        Self {
            sysname: Self::from_str("Starry"),
            nodename: Self::from_str("Starry - machine[0]"),
            release: Self::from_str("10.0.0"),
            version: Self::from_str("10.0.0"),
            machine: Self::from_str("10.0.0"),
            domainname: Self::from_str("https://github.com/BattiestStone4/Starry-On-ArceOS"),
        }
    }
}

impl UtsName {
    fn from_str(info: &str) -> [u8; 65] {
        let mut data: [u8; 65] = [0; 65];
        data[..info.len()].copy_from_slice(info.as_bytes());
        data
    }
}

pub fn sys_uname(name: UserPtr<UtsName>) -> LinuxResult<isize> {
    unsafe { *name.get()? = UtsName::default() };
    Ok(0)
}

pub fn sys_geteuid() -> LinuxResult<isize> {
    Ok(1000)
}

pub fn sys_getegid() -> LinuxResult<isize> {
    Ok(1000)
}

pub fn sys_utimensat(fd:isize, path:UserConstPtr<c_char>, times: UserPtr<[api::ctypes::timespec;2]>, flags: usize) -> LinuxResult<isize> {
    let path = path.get_as_null_terminated();
    let current_us = axhal::time::monotonic_time_nanos() as isize;
    let tv_sec= current_us / 1_000_000_000;
    let tv_nsec= current_us % 1_000_000_000;
    let mut atime = [tv_sec, tv_nsec];
    let mut mtime = [tv_sec, tv_nsec];
    
    if let Ok(time) = times.get(){
        unsafe{
            if (*time)[0].tv_nsec != UTIME_OMIT.try_into().unwrap() && (*time)[0].tv_nsec != UTIME_NOW.try_into().unwrap(){
                atime[0] = (*time)[0].tv_sec as isize;
                atime[1] = (*time)[0].tv_nsec as isize;
            }else if (*time)[0].tv_nsec == UTIME_OMIT.try_into().unwrap(){
                atime[1] = -1;
            }
            if (*time)[1].tv_nsec != UTIME_OMIT.try_into().unwrap() && (*time)[1].tv_nsec != UTIME_NOW.try_into().unwrap(){
                mtime[0] = (*time)[1].tv_sec as isize;
                mtime[1] = (*time)[1].tv_nsec as isize;
            }else if (*time)[1].tv_nsec == UTIME_OMIT.try_into().unwrap(){
                mtime[1] = -1;
            }
        }
    }else{
        return Ok(0)
    }
    if let Ok(path) = path{
        let path = handle_file_path(fd, Some(path.as_ptr() as _), true)?;
        let path = path.as_str();
        if path == "/dev/null/invalid" {
            return Ok(0);
        }
        if let Ok(file) = arceos_posix_api::open_file(fd as isize, path){
            arceos_posix_api::sys_utime(Arc::new(file), atime, mtime);
        }
    }else{
        if fd < 0{
            return Ok(0);
        }
        let file = File::from_fd(fd as c_int).unwrap();
        arceos_posix_api::sys_utime(file, atime, mtime);
    }
    Ok(0)
}