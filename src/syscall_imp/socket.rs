use core::ffi::{c_int, c_void};
use arceos_posix_api::ctypes::{self, sockaddr};
use axerrno::LinuxResult;

use crate::ptr::{PtrWrapper, UserConstPtr, UserPtr};

pub fn sys_socket(domain: usize, net_type: usize, protocol: usize) -> LinuxResult<isize>{
    Ok(arceos_posix_api::sys_socket(domain as c_int, net_type as c_int, protocol as c_int) as isize)
}

pub fn sys_bind(socket_fd: c_int, socket_addr: UserConstPtr<ctypes::sockaddr>, addrlen: u32)-> LinuxResult<isize>{
    let socket_addr = socket_addr.get().unwrap();
    Ok(arceos_posix_api::sys_bind(socket_fd, socket_addr, addrlen).try_into().unwrap())
}

pub fn sys_getsockname(sock_fd: i32, addr: UserPtr<ctypes::sockaddr>, addrlen:UserPtr<ctypes::socklen_t>)-> LinuxResult<isize>{
    let addr = addr.get().unwrap();
    let addrlen = addrlen.get().unwrap();
    Ok(unsafe{arceos_posix_api::sys_getsockname(sock_fd, addr, addrlen)} as isize) 
}

pub fn sys_setsockopt(_socket: usize, _level: usize, _optname: usize, _optval: usize, _optlen: usize) -> LinuxResult<isize>{
    Ok(0)
}

pub fn sys_sendto(socket_fd: i32, buf_ptr:UserConstPtr<c_void>, len:usize, flag:i32, socket_addr: UserConstPtr<ctypes::sockaddr>, addrlen: u32) -> LinuxResult<isize>{
    let buf_ptr = buf_ptr.get().unwrap();
    let socket_addr = socket_addr.get().unwrap();
    Ok(arceos_posix_api::sys_sendto(socket_fd, buf_ptr, len, flag, socket_addr, addrlen) as isize)
} 