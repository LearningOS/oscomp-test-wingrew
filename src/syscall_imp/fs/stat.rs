use core::ffi::c_char;

use alloc::ffi::CString;
use arceos_posix_api::ctypes::stat;
use axerrno::{LinuxError, LinuxResult};
use macro_rules_attribute::apply;

use crate::{
    ptr::{PtrWrapper, UserConstPtr, UserPtr},
    syscall_imp::syscall_instrument,
};

#[repr(C)]
#[derive(Debug)]
#[cfg(not(target_arch = "x86_64"))]
pub struct Kstat {
    /// 设备
    pub st_dev: u64,
    /// inode 编号
    pub st_ino: u64,
    /// 文件类型
    pub st_mode: u32,
    /// 硬链接数
    pub st_nlink: u32,
    /// 用户id
    pub st_uid: u32,
    /// 用户组id
    pub st_gid: u32,
    /// 设备号
    pub st_rdev: u64,
    /// padding
    pub _pad0: u64,
    /// 文件大小
    pub st_size: u64,
    /// 块大小
    pub st_blksize: u32,
    /// padding
    pub _pad1: u32,
    /// 块个数
    pub st_blocks: u64,
    /// 最后一次访问时间(秒)
    pub st_atime_sec: isize,
    /// 最后一次访问时间(纳秒)
    pub st_atime_nsec: isize,
    /// 最后一次修改时间(秒)
    pub st_mtime_sec: isize,
    /// 最后一次修改时间(纳秒)
    pub st_mtime_nsec: isize,
    /// 最后一次改变状态时间(秒)
    pub st_ctime_sec: isize,
    /// 最后一次改变状态时间(纳秒)
    pub st_ctime_nsec: isize,
}

#[repr(C)]
#[derive(Debug)]
#[cfg(target_arch = "x86_64")]
pub struct Kstat {
    pub st_dev: u64,       // 设备号
    pub st_ino: u64,       // inode
    pub st_nlink: u64,     // 文件links
    pub st_mode: u32, // 设备mode
    pub st_uid: u32,       // 文件uid
    pub st_gid: u32,       // 文件gid
    pub _pad0: u32,
    pub st_rdev: u64,       // 文件rdev
    pub st_size: u64,       // 文件大小
    pub st_blksize: u32,    // 占用块大小
    pub _pad1: u32,     // 保留
    pub st_blocks: u64,     // 占用块数量
    /// 最后一次访问时间(秒)
    pub st_atime_sec: isize,
    /// 最后一次访问时间(纳秒)
    pub st_atime_nsec: isize,
    /// 最后一次修改时间(秒)
    pub st_mtime_sec: isize,
    /// 最后一次修改时间(纳秒)
    pub st_mtime_nsec: isize,
    /// 最后一次改变状态时间(秒)
    pub st_ctime_sec: isize,
    /// 最后一次改变状态时间(纳秒)
    pub st_ctime_nsec: isize,
}

impl From<arceos_posix_api::ctypes::stat> for Kstat {
    fn from(stat: arceos_posix_api::ctypes::stat) -> Self {
        Self {
            st_dev: stat.st_dev,
            st_ino: stat.st_ino,
            st_mode: stat.st_mode,
            #[cfg(not(target_arch = "x86_64"))]
            st_nlink: stat.st_nlink,
            #[cfg(target_arch = "x86_64")]
            st_nlink: stat.st_nlink as u64,
            st_uid: stat.st_uid,
            st_gid: stat.st_gid,
            st_rdev: stat.st_rdev,
            _pad0: 0,
            st_size: stat.st_size as u64,
            st_blksize: stat.st_blksize as u32,
            _pad1: 0,
            st_blocks: stat.st_blocks as u64,
            st_atime_sec: stat.st_atime.tv_sec as isize,
            st_atime_nsec: stat.st_atime.tv_nsec as isize,
            st_mtime_sec: stat.st_mtime.tv_sec as isize,
            st_mtime_nsec: stat.st_mtime.tv_nsec as isize,
            st_ctime_sec: stat.st_ctime.tv_sec as isize,
            st_ctime_nsec: stat.st_ctime.tv_nsec as isize,
        }
    }
}

pub fn sys_fstat(fd: i32, kstatbuf: UserPtr<Kstat>) -> LinuxResult<isize> {
    let kstatbuf = kstatbuf.get()?;
    let mut statbuf = arceos_posix_api::ctypes::stat::default();

    let result = unsafe {
        arceos_posix_api::sys_fstat(fd, &mut statbuf as *mut arceos_posix_api::ctypes::stat)
    };
    if result < 0 {
        return Ok(result as _);
    }

    unsafe {
        let kstat = Kstat::from(statbuf);
        kstatbuf.write(kstat);
    }
    Ok(0)
}

#[apply(syscall_instrument)]
pub fn sys_fstatat(
    dir_fd: isize,
    path: UserConstPtr<c_char>,
    kstatbuf: UserPtr<Kstat>,
    _flags: i32,
) -> LinuxResult<isize> {
    let path = path.get_as_null_terminated()?;
    let path = arceos_posix_api::handle_file_path(dir_fd, Some(path.as_ptr() as _), false)?;
    let kstatbuf = kstatbuf.get()?;

    let mut statbuf = arceos_posix_api::ctypes::stat::default();
    let result = unsafe {
        arceos_posix_api::sys_stat(
            CString::new(path.as_str()).unwrap().as_ptr(),
            &mut statbuf as *mut arceos_posix_api::ctypes::stat,
        )
    };
    if result < 0 {
        return Ok(result as _);
    }

    unsafe {
        let kstat = Kstat::from(statbuf);
        kstatbuf.write(kstat);
    }

    Ok(0)
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct FsStatxTimestamp {
    pub tv_sec: i64,
    pub tv_nsec: u32,
}

/// statx - get file status (extended)
/// Standard C library (libc, -lc)
/// <https://man7.org/linux/man-pages/man2/statx.2.html>
#[repr(C)]
#[derive(Debug, Default)]
pub struct StatX {
    /// Bitmask of what information to get.
    pub stx_mask: u32,
    /// Block size for filesystem I/O.
    pub stx_blksize: u32,
    /// File attributes.
    pub stx_attributes: u64,
    /// Number of hard links.
    pub stx_nlink: u32,
    /// User ID of owner.
    pub stx_uid: u32,
    /// Group ID of owner.
    pub stx_gid: u32,
    /// File mode (permissions).
    pub stx_mode: u16,
    /// Inode number.
    pub stx_ino: u64,
    /// Total size, in bytes.
    pub stx_size: u64,
    /// Number of 512B blocks allocated.
    pub stx_blocks: u64,
    /// Mask to show what's supported in stx_attributes.
    pub stx_attributes_mask: u64,
    /// Last access timestamp.
    pub stx_atime: FsStatxTimestamp,
    /// Birth (creation) timestamp.
    pub stx_btime: FsStatxTimestamp,
    /// Last status change timestamp.
    pub stx_ctime: FsStatxTimestamp,
    /// Last modification timestamp.
    pub stx_mtime: FsStatxTimestamp,
    /// Major device ID (if special file).
    pub stx_rdev_major: u32,
    /// Minor device ID (if special file).
    pub stx_rdev_minor: u32,
    /// Major device ID of file system.
    pub stx_dev_major: u32,
    /// Minor device ID of file system.
    pub stx_dev_minor: u32,
    /// Mount ID.
    pub stx_mnt_id: u64,
    /// Memory alignment for direct I/O.
    pub stx_dio_mem_align: u32,
    /// Offset alignment for direct I/O.
    pub stx_dio_offset_align: u32,
}

#[apply(syscall_instrument)]
pub fn sys_statx(
    dirfd: i32,
    pathname: UserConstPtr<c_char>,
    flags: u32,
    _mask: u32,
    statxbuf: UserPtr<StatX>,
) -> LinuxResult<isize> {
    // `statx()` uses pathname, dirfd, and flags to identify the target
    // file in one of the following ways:

    // An absolute pathname(situation 1)
    //        If pathname begins with a slash, then it is an absolute
    //        pathname that identifies the target file.  In this case,
    //        dirfd is ignored.

    // A relative pathname(situation 2)
    //        If pathname is a string that begins with a character other
    //        than a slash and dirfd is AT_FDCWD, then pathname is a
    //        relative pathname that is interpreted relative to the
    //        process's current working directory.

    // A directory-relative pathname(situation 3)
    //        If pathname is a string that begins with a character other
    //        than a slash and dirfd is a file descriptor that refers to
    //        a directory, then pathname is a relative pathname that is
    //        interpreted relative to the directory referred to by dirfd.
    //        (See openat(2) for an explanation of why this is useful.)

    // By file descriptor(situation 4)
    //        If pathname is an empty string (or NULL since Linux 6.11)
    //        and the AT_EMPTY_PATH flag is specified in flags (see
    //        below), then the target file is the one referred to by the
    //        file descriptor dirfd.

    let path = pathname.get_as_str()?;
    let mut status = arceos_posix_api::ctypes::stat::default();
    const AT_EMPTY_PATH: u32 = 0x1000;
    if path.is_empty() {
        if flags & AT_EMPTY_PATH == 0 {
            return Err(LinuxError::EINVAL);
        }
        let res = unsafe { arceos_posix_api::sys_fstat(dirfd, &mut status as *mut _) };
        if res < 0 {
            return Err(LinuxError::try_from(-res).unwrap());
        }
    } else {
        unsafe {arceos_posix_api::sys_stat(CString::new(path).unwrap().as_ptr(), &mut status as *mut _);}
    }
    let statx = unsafe { &mut *statxbuf.get()? };
    statx.stx_blksize = status.st_blksize as u32;
    statx.stx_attributes = status.st_mode as u64;
    statx.stx_nlink = status.st_nlink as u32;
    statx.stx_uid = status.st_uid;
    statx.stx_gid = status.st_gid;
    statx.stx_mode = status.st_mode as u16;
    statx.stx_ino = status.st_ino;
    statx.stx_size = status.st_size as u64;
    statx.stx_blocks = status.st_blocks as u64;
    statx.stx_attributes_mask = 0x7FF;
    statx.stx_atime.tv_sec = status.st_atime.tv_sec as i64;
    statx.stx_atime.tv_nsec = status.st_atime.tv_nsec as u32;
    statx.stx_ctime.tv_sec = status.st_ctime.tv_sec as i64;
    statx.stx_ctime.tv_nsec = status.st_ctime.tv_nsec as u32;
    statx.stx_mtime.tv_sec = status.st_mtime.tv_sec as i64;
    statx.stx_mtime.tv_nsec = status.st_mtime.tv_nsec as u32;
    Ok(0)
}


#[repr(C)]
#[derive(Debug, Default)]
pub struct Statfs {
    pub f_ftype: u64,   // 文件系统的类型
    pub f_bsize: u64,   // 经优化后的传输块的大小
    pub f_blocks: u64,  // 文件系统数据块总数
    pub f_bfree: u64,   // 可用块数
    pub f_bavail: u64,  // 普通用户能够获得的块数
    pub f_files: u64,   // 文件结点总数
    pub f_ffree: u64,   // 可用文件结点数
    pub f_fsid: u64,    // 文件系统标识
    pub f_namelen: u64, // 文件名的最大长度
}

#[apply(syscall_instrument)]
pub fn sys_statfs(
    pathname: UserConstPtr<c_char>,
    statfsbuf: UserPtr<Statfs>,
) -> LinuxResult<isize> {
    let path = pathname.get_as_str()?;
    let _path = arceos_posix_api::handle_file_path(-1, Some(path.as_ptr() as _), false)?;
    let statfs = unsafe { &mut *statfsbuf.get()? };
    statfs.f_bsize = 4096;
    statfs.f_blocks = 255;
    statfs.f_bfree = 0;
    statfs.f_bavail = 0;
    statfs.f_files = 1;
    statfs.f_ffree = 0;
    statfs.f_fsid = 0;
    statfs.f_namelen = 255;
    statfs.f_ftype = 0;
    // statfs.f_spare = [0; 6];
    info!("statfs:{:?}", statfs);
    Ok(0)
}

#[apply(syscall_instrument)]
pub fn sys_stat(
    path: UserConstPtr<c_char>, 
    kstatbuf: UserPtr<Kstat>,) -> LinuxResult<isize> {
    let kstatbuf = kstatbuf.get()?;
    let mut statbuf = arceos_posix_api::ctypes::stat::default();
    unsafe{
        arceos_posix_api::sys_stat(
            path.get().unwrap(),
            &mut statbuf as *mut _
        );
    }
    unsafe {
        let kstat = Kstat::from(statbuf);
        kstatbuf.write(kstat);
    }
    Ok(0)
}