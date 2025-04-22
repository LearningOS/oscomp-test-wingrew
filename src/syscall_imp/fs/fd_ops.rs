use core::ffi::c_int;

use arceos_posix_api as api;
use axerrno::{LinuxError, LinuxResult};
use crate::task::get_fdlimit;

pub fn sys_dup(old_fd: c_int) -> LinuxResult<isize> {
    let limit = get_fdlimit();
    info!("limit:{}, old_fd:{}", limit, old_fd);
    if api::get_table_count() < limit as usize{
        Ok(api::sys_dup(old_fd) as _)
    }else{
        Err(LinuxError::EMFILE)
    }
}

pub fn sys_dup3(old_fd: c_int, new_fd: c_int) -> LinuxResult<isize> {
    Ok(api::sys_dup2(old_fd, new_fd) as _)
}

pub fn sys_close(fd: c_int) -> LinuxResult<isize> {
    Ok(api::sys_close(fd) as _)
}

pub fn sys_fcntl(fd: c_int, cmd: c_int, arg: usize) -> LinuxResult<isize> {
    Ok(api::sys_fcntl(fd, cmd, arg) as _)
}
