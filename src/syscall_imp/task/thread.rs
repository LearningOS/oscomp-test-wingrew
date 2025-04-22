use core::{ffi::{c_char, c_int}, ptr};

use alloc::vec::Vec;
use arceos_posix_api::{query_futex, add_futex, remove_futex};
use axerrno::{LinuxError, LinuxResult};
use axtask::{TaskExtRef, current, yield_now};
use macro_rules_attribute::apply;
use num_enum::TryFromPrimitive;
use crate::{
    ctypes::{FluxStatus, WaitFlags, WaitStatus},
    ptr::{PtrWrapper, UserConstPtr, UserPtr},
    syscall_imp::syscall_instrument,
    task::{get_fdlimit, wait_pid},
};

/// ARCH_PRCTL codes
///
/// It is only avaliable on x86_64, and is not convenient
/// to generate automatically via c_to_rust binding.
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum ArchPrctlCode {
    /// Set the GS segment base
    SetGs = 0x1001,
    /// Set the FS segment base
    SetFs = 0x1002,
    /// Get the FS segment base
    GetFs = 0x1003,
    /// Get the GS segment base
    GetGs = 0x1004,
    /// The setting of the flag manipulated by ARCH_SET_CPUID
    GetCpuid = 0x1011,
    /// Enable (addr != 0) or disable (addr == 0) the cpuid instruction for the calling thread.
    SetCpuid = 0x1012,
}



#[apply(syscall_instrument)]
pub fn sys_getpid() -> LinuxResult<isize> {
    Ok(axtask::current().task_ext().proc_id as _)
}

#[apply(syscall_instrument)]
pub fn sys_gettid() -> LinuxResult<isize> {
    Ok(axtask::current().id().get_id() as isize)
}

#[apply(syscall_instrument)]
pub fn sys_prlimit64(
    _pid: i32,
    _resource: i32,
    _new_limit: UserConstPtr<arceos_posix_api::ctypes::rlimit>,
    _old_limit: UserPtr<arceos_posix_api::ctypes::rlimit>,
) -> LinuxResult<isize> {
    // warn!("sys_prlimit64: not implemented");
    let new_limit = _new_limit.get();
    let _old_limit = _old_limit.get();
    if let Ok(new_limit) = new_limit{
        match _resource as u32 {
            arceos_posix_api::ctypes::RLIMIT_DATA => {}
            arceos_posix_api::ctypes::RLIMIT_STACK => {}
            arceos_posix_api::ctypes::RLIMIT_NOFILE => {
                let task = axtask::current();
                let limit = unsafe { *new_limit };
                task.task_ext().set_fdlimit(limit.rlim_cur as _);
                // let mut task = axtask::current().task_ext();
                // task.set_fd_limit(unsafe{(*rlimits).rlim_cur as _});
            }
            _ => return Err(LinuxError::EINVAL),
        }
    }
    if let Ok(old_limit) = _old_limit{
        match _resource as u32 {
            arceos_posix_api::ctypes::RLIMIT_DATA => {}
            arceos_posix_api::ctypes::RLIMIT_STACK => {}
            arceos_posix_api::ctypes::RLIMIT_NOFILE => {
                let limit = get_fdlimit();
                unsafe { (*old_limit).rlim_cur = limit;
                        (*old_limit).rlim_max = limit;}
            }
            _ => return Err(LinuxError::EINVAL),
        }
    }

    // unsafe{arceos_posix_api::sys_setrlimit(_resource as c_int, new_limit);}
    // info!(
    //     "sys_prlimit64: pid: {}, resource: {}, new_limit: {:#x}, old_limit: {:#x}",
    //     _pid, _resource, new_limit, old_limit
    // );
    Ok(0)
}

#[apply(syscall_instrument)]
pub fn sys_getppid() -> LinuxResult<isize> {
    Ok(axtask::current().task_ext().get_parent() as _)
}

pub fn sys_exit(status: i32) -> ! {
    let curr = current();
    let clear_child_tid = curr.task_ext().clear_child_tid() as *mut i32;
    if !clear_child_tid.is_null() {
        // TODO: check whether the address is valid
        unsafe {
            // TODO: Encapsulate all operations that access user-mode memory into a unified function
            *(clear_child_tid) = 0;
        }
        // TODO: wake up threads, which are blocked by futex, and waiting for the address pointed by clear_child_tid
    }
    axtask::exit(status);
}

pub fn sys_exit_group(status: i32) -> ! {
    warn!("Temporarily replace sys_exit_group with sys_exit");
    axtask::exit(status);
}

/// To set the clear_child_tid field in the task extended data.
///
/// The set_tid_address() always succeeds
#[apply(syscall_instrument)]
pub fn sys_set_tid_address(tid_ptd: UserConstPtr<i32>) -> LinuxResult<isize> {
    info!("sys_set_tid_address: tid_ptd: {:#x}", tid_ptd.address().as_usize());
    let curr = current();
    curr.task_ext()
        .set_clear_child_tid(tid_ptd.address().as_ptr() as _);
    Ok(curr.id().as_u64() as isize)
}

#[cfg(target_arch = "x86_64")]
#[apply(syscall_instrument)]
pub fn sys_arch_prctl(code: i32, addr: UserPtr<u64>) -> LinuxResult<isize> {
    use axerrno::LinuxError;
    match ArchPrctlCode::try_from(code).map_err(|_| LinuxError::EINVAL)? {
        // According to Linux implementation, SetFs & SetGs does not return
        // error at all
        ArchPrctlCode::SetFs => {
            unsafe {
                axhal::arch::write_thread_pointer(addr.address().as_usize());
            }
            Ok(0)
        }
        ArchPrctlCode::SetGs => {
            unsafe {
                x86::msr::wrmsr(x86::msr::IA32_KERNEL_GSBASE, addr.address().as_usize() as _);
            }
            Ok(0)
        }
        ArchPrctlCode::GetFs => {
            unsafe {
                *addr.get()? = axhal::arch::read_thread_pointer() as u64;
            }
            Ok(0)
        }

        ArchPrctlCode::GetGs => {
            unsafe {
                *addr.get()? = x86::msr::rdmsr(x86::msr::IA32_KERNEL_GSBASE);
            }
            Ok(0)
        }
        ArchPrctlCode::GetCpuid => Ok(0),
        ArchPrctlCode::SetCpuid => Err(LinuxError::ENODEV),
    }
}

#[apply(syscall_instrument)]
pub fn sys_clone(
    flags: usize,
    user_stack: usize,
    ptid: usize,
    arg3: usize,
    arg4: usize,
) -> LinuxResult<isize> {
    let tls = arg3;
    let ctid = arg4;
    let curr_task = current();    
    let stack = if user_stack == 0 {
        None
    } else {
        Some(user_stack)
    };
    if let Ok(new_task_id) = curr_task
        .task_ext()
        .clone_task(flags, stack, ptid, tls, ctid)
    {
        Ok(new_task_id as isize)
    } else {
        Err(LinuxError::ENOMEM)
    }
}

#[apply(syscall_instrument)]
pub fn sys_wait4(pid: i32, exit_code_ptr: UserPtr<i32>, option: u32) -> LinuxResult<isize> {
    let option_flag = WaitFlags::from_bits(option).unwrap();
    let exit_code_ptr = exit_code_ptr.nullable(UserPtr::get)?;
    loop {
        let answer = wait_pid(pid, exit_code_ptr.unwrap_or_else(ptr::null_mut));
        match answer {
            Ok(pid) => {
                return Ok(pid as isize);
            }
            Err(status) => match status {
                WaitStatus::NotExist => {
                    return Err(LinuxError::ECHILD);
                }
                WaitStatus::Running => {
                    if option_flag.contains(WaitFlags::WNOHANG) {
                        return Ok(0);
                    } else {

                        yield_now();
                    }
                }
                _ => {
                    panic!("Shouldn't reach here!");
                }
            },
        }
    }
}

#[apply(syscall_instrument)]
pub fn sys_execve(
    path: UserConstPtr<c_char>,
    argv: UserConstPtr<usize>,
    envp: UserConstPtr<usize>,
) -> LinuxResult<isize> {
    let path_str = path.get_as_str()?;

    let args = argv
        .get_as_null_terminated()?
        .iter()
        .map(|arg| {
            UserConstPtr::<c_char>::from(*arg)
                .get_as_str()
                .map(Into::into)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let envs = envp
        .get_as_null_terminated()?
        .iter()
        .map(|env| {
            UserConstPtr::<c_char>::from(*env)
                .get_as_str()
                .map(Into::into)
        })
        .collect::<Result<Vec<_>, _>>()?;

    info!(
        "execve: path: {:?}, args: {:?}, envs: {:?}",
        path_str, args, envs
    );

    if let Err(e) = crate::task::exec(path_str, &args, &envs) {
        error!("Failed to exec: {:?}", e);
        return Err::<isize, _>(LinuxError::ENOSYS);
    }

    unreachable!("execve should never return");
}

pub fn sys_futex( 
    futex: UserPtr<i32>,
    _op: usize,
    _val: usize,
    _timeout: usize,
    newval: usize,
    _val3: usize,
) -> LinuxResult<isize> {
    let futex = futex.get();
    let flux_status = FluxStatus::try_from(_op as i32).unwrap();
    info!("flux_status:{:?}, futex:{:?}, _op:{:?}, _val:{:?}, id:{}, time:{:?}", flux_status, futex, _op, _val as i32, sys_gettid().unwrap(), _timeout);
    match flux_status {
        FluxStatus::WaitPrivate => {
            unsafe {
                info!("real_futex:{}", *futex.unwrap());
                if *(futex.unwrap()) == _val as i32{
                    add_futex(futex.unwrap() as usize, sys_gettid().unwrap() as usize);
                    // WaitPrivate
                    loop{
                        if query_futex(futex.unwrap() as usize, sys_gettid().unwrap() as usize){
                            yield_now();
                        }else{
                            return Ok(0);
                        }
                    }
                }else{
                    return Ok(0);
                }
            }
        }
        FluxStatus::WakePrivate => {
            // WakePrivate
            return Ok(remove_futex(_val) as isize);
        }
        _ => {
            return Err(LinuxError::ENOSYS);
        }
    }
}