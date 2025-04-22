use crate::ctypes;
use axerrno::{LinuxError, LinuxResult};
use core::ffi::{c_int, c_void};

#[cfg(feature = "fd")]
use crate::imp::fd_ops::get_file_like;
#[cfg(not(feature = "fd"))]
use axio::prelude::*;

/// Read data from the file indicated by `fd`.
///
/// Return the read size if success.
pub fn sys_read(fd: c_int, buf: *mut c_void, count: usize) -> ctypes::ssize_t {
    debug!("sys_read <= {} {:#x} {}", fd, buf as usize, count);
    syscall_body!(sys_read, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let dst = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };

        #[cfg(feature = "fd")]
        {   
            let file = get_file_like(fd)?;
            let result = file.read(dst)?;
            Ok(result as ctypes::ssize_t)
        }
        #[cfg(not(feature = "fd"))]
        match fd {
            0 => Ok(super::stdio::stdin().read(dst)? as ctypes::ssize_t),
            1 | 2 => Err(LinuxError::EPERM),
            _ => Err(LinuxError::EBADF),
        }
    })
}

pub fn sys_pread64(
    fd: c_int,
    buf: *mut c_void,
    count: usize,
    offset: u64,
) -> ctypes::ssize_t {
    debug!("sys_pread64 <= {} {:#x} {} {}", fd, buf as usize, count, offset);
    syscall_body!(sys_pread64, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let dst = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };
        {
            let file = get_file_like(fd)?;
            let result = file.read_at(offset, dst)?;
            Ok(result as ctypes::ssize_t)
        }
    })
}

fn write_impl(fd: c_int, buf: *const c_void, count: usize) -> LinuxResult<ctypes::ssize_t> {
    if buf.is_null() {
        return Err(LinuxError::EFAULT);
    }
    let src = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };
    #[cfg(feature = "fd")]
    {
        Ok(get_file_like(fd)?.write(src)? as ctypes::ssize_t)
    }
    #[cfg(not(feature = "fd"))]
    match fd {
        0 => Err(LinuxError::EPERM),
        1 | 2 => Ok(super::stdio::stdout().write(src)? as ctypes::ssize_t),
        _ => Err(LinuxError::EBADF),
    }
}

/// Write data to the file indicated by `fd`.
///
/// Return the written size if success.
pub fn sys_write(fd: c_int, buf: *const c_void, count: usize) -> ctypes::ssize_t {
    debug!("sys_write <= {} {:#x} {}", fd, buf as usize, count);
    syscall_body!(sys_write, write_impl(fd, buf, count))
}

/// Write a vector.
pub unsafe fn sys_writev(fd: c_int, iov: *const ctypes::iovec, iocnt: c_int) -> ctypes::ssize_t {
    debug!("sys_writev <= fd: {} {:?} {}", fd, iov, iocnt);
    syscall_body!(sys_writev, {
        if !(0..=1024).contains(&iocnt) {
            return Err(LinuxError::EINVAL);
        }
        let iovs = unsafe { core::slice::from_raw_parts(iov, iocnt as usize) };
        let mut ret = 0;
        info!("sys_writev <= fd: {} iovs: {:?}", fd, iovs);
        for iov in iovs.iter() {
            if iov.iov_len == 0 {
                continue;
            }
            let result = write_impl(fd, iov.iov_base, iov.iov_len)?;
            ret += result;

            if result < iov.iov_len as isize {
                break;
            }
        }
        Ok(ret)
    })
}

/// Read a vector.
pub unsafe fn sys_readv(fd: c_int, iov: *const ctypes::iovec, iocnt: c_int) -> ctypes::ssize_t {
    debug!("sys_readv <= fd: {} {:?} {}", fd, iov, iocnt);
    syscall_body!(sys_writev, {
        if !(0..=1024).contains(&iocnt) {
            return Err(LinuxError::EINVAL);
        }
        let iovs = unsafe { core::slice::from_raw_parts(iov, iocnt as usize) };
        let mut ret = 0;
        info!("sys_writev <= fd: {} iovs: {:?}", fd, iovs);
        for iov in iovs.iter() {
            if iov.iov_len == 0 {
                continue;
            }
            let dst = unsafe { core::slice::from_raw_parts_mut(iov.iov_base as *mut u8, iov.iov_len) };
            let file = get_file_like(fd)?;
            let result = file.read(dst)?;
            ret += result;
            if result < iov.iov_len{
                break;
            }
        }
        Ok(ret)
    })
}
