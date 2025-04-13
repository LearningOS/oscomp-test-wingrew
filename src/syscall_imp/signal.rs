use core::ffi::c_void;

use axerrno::LinuxResult;
use axhal::trap::DEAL_SIGNAL;
use axtask::{current, yield_now, TaskExtRef};
use crate::syscall_imp::register_trap_handler;
use crate::{ctypes::{SigAction, SignalFlags, SignalSet}, ptr::{PtrWrapper, UserConstPtr, UserPtr}, task::get_task_by_id};


#[register_trap_handler(DEAL_SIGNAL)]
pub fn dealwith_signal(){
    loop{
        check_pending_signals();
        let(frozen, kill) = {
            let curr = current();
            (curr.task_ext().frozen, curr.task_ext().killed)
        };
        if !frozen || !kill{
            break;
        }
        yield_now();
    }
}

fn check_pending_signals(){

}
pub fn sys_rt_sigprocmask(
    _how: i32,
    _set: UserPtr<u64>,
    _oldset: UserPtr<u64>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    let curr = current();
    let new_set = _set.get();
    let old_set = _oldset.get();
    match old_set{
        Ok(old_set) => {
            unsafe{ *old_set = curr.task_ext().get_mask().bits();}
            info!("old_set: {:?}", curr.task_ext().get_mask());            
        }
        Err(e) => {
            info!("old_set: {:?}", e);
        }
    }
    match new_set {
        Ok(new_set) => {
            let flag = SignalFlags::from_bits(unsafe {*new_set} as u64).unwrap();
            info!("new_set: {:?}", flag);
            match SignalSet::try_from(_how).unwrap() {
                SignalSet::SigBlock => {
                    // Block signals
                    curr
                    .task_ext().add_mask(flag);
                }
                SignalSet::SigUnblock => {
                    // Unblock signals
                    curr
                    .task_ext().del_mask(flag);
                }
                SignalSet::SigSetmask => {
                    // Set the signal mask
                    curr.task_ext().set_mask(flag);
                }
            }
        }
        Err(e) => {
            info!("new_set: {:?}", e);
        }
    }

    Ok(0)
}

pub fn sys_rt_sigaction(
    _signum: i32,
    _act: UserConstPtr<SigAction>,
    _oldact: UserPtr<SigAction>,
    _sigsetsize: usize,
) -> LinuxResult<isize> {
    let curr = current();
    let new_act = _act.get();   
    let old_act = _oldact.get();
    if let Some(signal) = SignalFlags::from_bits(_signum as u64) {
        if signal != SignalFlags::SIGKILL && signal != SignalFlags::SIGSTOP {
            if let Ok(new_act) = new_act {
                if let Ok(old_act) = old_act {
                    unsafe { *old_act = curr.task_ext().get_signal_action(_signum as usize) }
                }
                curr.task_ext().set_signal_action(_signum as usize, new_act);
            } else {
                return Err(axerrno::LinuxError::EINVAL);
            }
        } 
    }
    // warn!("sys_rt_sigaction: not implemented");
    Ok(0)
}

pub fn sys_rt_sigtimedwait(
    _signum: i32,
    _act: UserConstPtr<c_void>,
    _oldact: UserPtr<c_void>,
) -> LinuxResult<isize> {
    warn!("sys_rt_sigaction: not implemented");
    Ok(0)
}

pub fn sys_rt_sigreturn() -> LinuxResult<isize> {
    
    Ok(0)
}

pub fn sys_kill(_pid: usize, _sig: i32) -> LinuxResult<isize> {
    // warn!("sys_kill: not implemented");
    if let Some(task) = get_task_by_id(_pid as _) {
        let _sig = SignalFlags::from_bits(_sig as u64).unwrap();
        task.task_ext().add_mask(_sig as _);
    }else{
        warn!("sys_kill: task not found");
    }
    Ok(0)
}