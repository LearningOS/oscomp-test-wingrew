//! clone 任务时指定的参数。

pub const UTIME_NOW:usize = 0x3fffffff; 
pub const UTIME_OMIT:usize = 0x3FFFFFFE; 
use bitflags::*;
bitflags! {
    /// 用于 sys_clone 的选项
    #[derive(Debug, Clone, Copy)]
    pub struct CloneFlags: u32 {
        /// .
        const CLONE_NEWTIME = 1 << 7;
        /// 共享地址空间
        const CLONE_VM = 1 << 8;
        /// 共享文件系统新信息
        const CLONE_FS = 1 << 9;
        /// 共享文件描述符(fd)表
        const CLONE_FILES = 1 << 10;
        /// 共享信号处理函数
        const CLONE_SIGHAND = 1 << 11;
        /// 创建指向子任务的fd，用于 sys_pidfd_open
        const CLONE_PIDFD = 1 << 12;
        /// 用于 sys_ptrace
        const CLONE_PTRACE = 1 << 13;
        /// 指定父任务创建后立即阻塞，直到子任务退出才继续
        const CLONE_VFORK = 1 << 14;
        /// 指定子任务的 ppid 为当前任务的 ppid，相当于创建“兄弟”而不是“子女”
        const CLONE_PARENT = 1 << 15;
        /// 作为一个“线程”被创建。具体来说，它同 CLONE_PARENT 一样设置 ppid，且不可被 wait
        const CLONE_THREAD = 1 << 16;
        /// 子任务使用新的命名空间。目前还未用到
        const CLONE_NEWNS = 1 << 17;
        /// 子任务共享同一组信号量。用于 sys_semop
        const CLONE_SYSVSEM = 1 << 18;
        /// 要求设置 tls
        const CLONE_SETTLS = 1 << 19;
        /// 要求在父任务的一个地址写入子任务的 tid
        const CLONE_PARENT_SETTID = 1 << 20;
        /// 要求将子任务的一个地址清零。这个地址会被记录下来，当子任务退出时会触发此处的 futex
        const CLONE_CHILD_CLEARTID = 1 << 21;
        /// 历史遗留的 flag，现在按 linux 要求应忽略
        const CLONE_DETACHED = 1 << 22;
        /// 与 sys_ptrace 相关，目前未用到
        const CLONE_UNTRACED = 1 << 23;
        /// 要求在子任务的一个地址写入子任务的 tid
        const CLONE_CHILD_SETTID = 1 << 24;
        /// New pid namespace.
        const CLONE_NEWPID = 1 << 29;
    }

    pub struct WaitFlags: u32 {
        /// 不挂起当前进程，直接返回
        const WNOHANG = 1 << 0;
        /// 报告已执行结束的用户进程的状态
        const WIMTRACED = 1 << 1;
        /// 报告还未结束的用户进程的状态
        const WCONTINUED = 1 << 3;
        /// Wait for any child
        const WALL = 1 << 30;
        /// Wait for cloned process
        const WCLONE = 1 << 31;
    }

}



// 定义 sigset_t（1024 位 = 128 字节）
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SigsetT {
    bits: [u64; 16], // 16 个 u64，每个 64 位，总计 1024 位
}


// 定义 struct sigaction
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SigAction {
    pub sa_handler: usize,                    // void (*)(int)
    pub sa_flags: usize,                        // int
    pub sa_restorer: usize,
    pub sa_mask: usize,                       // sigset_t
}

impl Default for SigAction {
    fn default() -> Self {
        Self {
            sa_handler: 0,
            sa_flags: 0,
            sa_restorer: 0,
            sa_mask: 0,
        }
    }
}

// siginfo_t 的简化定义（根据需要扩展）
#[repr(C)]
pub struct SiginfoT {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    // 其他字段根据具体需求添加，例如 si_pid, si_uid 等
}

pub const VAILD_SIGNAL: usize = 64;

bitflags! {
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    pub struct SignalFlags: u64 {
        const	SIGHUP		= 1 << 0;
        /// Interactive attention signal.
        const	SIGINT		= 1 << 1;
        /// Quit.
        const	SIGQUIT		= 1 << 2;
        /// Illegal instruction.
        const	SIGILL		= 1 << 3;
        /// Trace/breakpoint trap.
        const	SIGTRAP		= 1 << 4;
        /// IOT instruction, abort() on a PDP-11.
        const	SIGABRT		= 1 << 5;
        /// Bus error.
        const	SIGBUS		= 1 << 6;
        /// Erroneous arithmetic operation.
        const	SIGFPE		= 1 << 7;
        /// Killed.
        const	SIGKILL		= 1 << 8;
        /// User-defined signal 1.
        const	SIGUSR1		= 1 << 9;
        /// Invalid access to storage.
        const	SIGSEGV		= 1 << 10;
        /// User-defined signal 2.
        const	SIGUSR2		= 1 << 11;
        /// Broken pipe.
        const	SIGPIPE		= 1 << 12;
        /// Alarm clock.
        const	SIGALRM		= 1 << 13;
        /// Termination request.
        const	SIGTERM		= 1 << 14;
        const	SIGSTKFLT	= 1 << 15;
        /// Child terminated or stopped.
        const	SIGCHLD		= 1 << 16;
        /// Continue.
        const	SIGCONT		= 1 << 17;
        /// Stop, unblockable.
        const	SIGSTOP		= 1 << 18;
        /// Keyboard stop.
        const	SIGTSTP		= 1 << 19;
        /// Background read from control terminal.
        const	SIGTTIN		= 1 << 20;
        /// Background write to control terminal.
        const	SIGTTOU		= 1 << 21;
        /// Urgent data is available at a socket.
        const	SIGURG		= 1 << 22;
        /// CPU time limit exceeded.
        const	SIGXCPU		= 1 << 23;
        /// File size limit exceeded.
        const	SIGXFSZ		= 1 << 24;
        /// Virtual timer expired.
        const	SIGVTALRM	= 1 << 25;
        /// Profiling timer expired.
        const	SIGPROF		= 1 << 26;
        /// Window size change (4.3 BSD, Sun).
        const	SIGWINCH	= 1 << 27;
        /// I/O now possible (4.2 BSD).
        const	SIGIO		= 1 << 28;
        const   SIGPWR      = 1 << 29;
        /// Bad system call.
        const   SIGSYS      = 1 << 30;
        /* --- realtime signals for pthread --- */
        const   SIGTIMER    = 1 << 31;
        const   SIGCANCEL   = 1 << 32;
        const   SIGSYNCCALL = 1 << 33;
        /* --- other realtime signals --- */
        const   SIGRT_3     = 1 << 34;
        const   SIGRT_4     = 1 << 35;
        const   SIGRT_5     = 1 << 36;
        const   SIGRT_6     = 1 << 37;
        const   SIGRT_7     = 1 << 38;
        const   SIGRT_8     = 1 << 39;
        const   SIGRT_9     = 1 << 40;
        const   SIGRT_10    = 1 << 41;
        const   SIGRT_11    = 1 << 42;
        const   SIGRT_12    = 1 << 43;
        const   SIGRT_13    = 1 << 44;
        const   SIGRT_14    = 1 << 45;
        const   SIGRT_15    = 1 << 46;
        const   SIGRT_16    = 1 << 47;
        const   SIGRT_17    = 1 << 48;
        const   SIGRT_18    = 1 << 49;
        const   SIGRT_19    = 1 << 50;
        const   SIGRT_20    = 1 << 51;
        const   SIGRT_21    = 1 << 52;
        const   SIGRT_22    = 1 << 53;
        const   SIGRT_23    = 1 << 54;
        const   SIGRT_24    = 1 << 55;
        const   SIGRT_25    = 1 << 56;
        const   SIGRT_26    = 1 << 57;
        const   SIGRT_27    = 1 << 58;
        const   SIGRT_28    = 1 << 59;
        const   SIGRT_29    = 1 << 60;
        const   SIGRT_30    = 1 << 61;
        const   SIGRT_31    = 1 << 62;
        const   SIGRTMAX    = 1 << 63;
    }
}

/// sys_wait4 的返回值
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitStatus {
    /// 子任务正常退出
    Exited,
    /// 子任务正在运行
    Running,
    /// 找不到对应的子任务
    NotExist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSet {
    SigBlock,
    SigUnblock,
    SigSetmask,
}

impl TryFrom<i32> for SignalSet {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SignalSet::SigBlock),
            1 => Ok(SignalSet::SigUnblock),
            2 => Ok(SignalSet::SigSetmask),
            _ => Err("Invalid signal set value"),
        }
    }
}

#[repr(i32)]
#[derive(Debug)]
pub enum FluxStatus {
    Wait,           // 0: FUTEX_WAIT
    Wake,           // 1: FUTEX_WAKE
    Requeue,        // 3: FUTEX_REQUEUE
    CmpRequeue,     // 4: FUTEX_CMP_REQUEUE
    WakeOp,         // 5: FUTEX_WAKE_OP
    LockPi,         // 6: FUTEX_LOCK_PI
    UnlockPi,       // 7: FUTEX_UNLOCK_PI
    WaitBitset,     // 9: FUTEX_WAIT_BITSET
    WakeBitset,     // 10: FUTEX_WAKE_BITSET
    WaitPrivate = 128,    // 128: FUTEX_WAIT_PRIVATE
    WakePrivate = 129,    // 129: FUTEX_WAKE_PRIVATE
    NONE = -1,
}

// 实现 TryFrom<i32> 以支持 i32 到 FluxStatus 的转换
impl TryFrom<i32> for FluxStatus {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FluxStatus::Wait),
            1 => Ok(FluxStatus::Wake),
            3 => Ok(FluxStatus::Requeue),
            4 => Ok(FluxStatus::CmpRequeue),
            5 => Ok(FluxStatus::WakeOp),
            6 => Ok(FluxStatus::LockPi),
            7 => Ok(FluxStatus::UnlockPi),
            9 => Ok(FluxStatus::WaitBitset),
            10 => Ok(FluxStatus::WakeBitset),
            128 => Ok(FluxStatus::WaitPrivate),
            129 => Ok(FluxStatus::WakePrivate),
            -1 => Ok(FluxStatus::NONE),
            _ => Err("Invalid value for FluxStatus"),
        }
    }
}



#[repr(C)]
pub struct Tms {
    /// 进程用户态执行时间，单位为us
    pub tms_utime: usize,
    /// 进程内核态执行时间，单位为us
    pub tms_stime: usize,
    /// 子进程用户态执行时间和，单位为us
    pub tms_cutime: usize,
    /// 子进程内核态执行时间和，单位为us
    pub tms_cstime: usize,
}

numeric_enum_macro::numeric_enum! {
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    pub enum TimerType {
    /// 表示目前没有任何计时器(不在linux规范中，是os自己规定的)
    NONE = -1,
    /// 统计系统实际运行时间
    REAL = 0,
    /// 统计用户态运行时间
    VIRTUAL = 1,
    /// 统计进程的所有用户态/内核态运行时间
    PROF = 2,
    }
}

impl From<usize> for TimerType {
    fn from(num: usize) -> Self {
        match Self::try_from(num as i32) {
            Ok(val) => val,
            Err(_) => Self::NONE,
        }
    }
}
pub struct TimeStat {
    utime_ns: usize,
    stime_ns: usize,
    user_timestamp: usize,
    kernel_timestamp: usize,
    timer_type: TimerType,
    timer_interval_ns: usize,
    timer_remained_ns: usize,
}

impl Default for TimeStat {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeStat {
    pub fn new() -> Self {
        Self {
            utime_ns: 0,
            stime_ns: 0,
            user_timestamp: 0,
            kernel_timestamp: 0,
            timer_type: TimerType::NONE,
            timer_interval_ns: 0,
            timer_remained_ns: 0,
        }
    }

    pub fn output(&self) -> (usize, usize) {
        (self.utime_ns, self.stime_ns)
    }

    pub fn reset(&mut self, current_timestamp: usize) {
        self.utime_ns = 0;
        self.stime_ns = 0;
        self.user_timestamp = 0;
        self.kernel_timestamp = current_timestamp;
    }

    pub fn switch_into_kernel_mode(&mut self, current_timestamp: usize) {
        let now_time_ns = current_timestamp;
        let delta = now_time_ns - self.kernel_timestamp;
        self.utime_ns += delta;
        self.kernel_timestamp = now_time_ns;
        if self.timer_type != TimerType::NONE {
            self.update_timer(delta);
        };
    }

    pub fn switch_into_user_mode(&mut self, current_timestamp: usize) {
        let now_time_ns = current_timestamp;
        let delta = now_time_ns - self.kernel_timestamp;
        self.stime_ns += delta;
        self.user_timestamp = now_time_ns;
        if self.timer_type == TimerType::REAL || self.timer_type == TimerType::PROF {
            self.update_timer(delta);
        }
    }

    pub fn switch_from_old_task(&mut self, current_timestamp: usize) {
        let now_time_ns = current_timestamp;
        let delta = now_time_ns - self.kernel_timestamp;
        self.stime_ns += delta;
        self.kernel_timestamp = now_time_ns;
        if self.timer_type == TimerType::REAL || self.timer_type == TimerType::PROF {
            self.update_timer(delta);
        }
    }

    pub fn switch_to_new_task(&mut self, current_timestamp: usize) {
        let now_time_ns = current_timestamp;
        let delta = now_time_ns - self.kernel_timestamp;
        self.kernel_timestamp = now_time_ns;
        if self.timer_type == TimerType::REAL {
            self.update_timer(delta);
        }
    }

    pub fn set_timer(
        &mut self,
        timer_interval_ns: usize,
        timer_remained_ns: usize,
        timer_type: usize,
    ) -> bool {
        self.timer_type = timer_type.into();
        self.timer_interval_ns = timer_interval_ns;
        self.timer_remained_ns = timer_remained_ns;
        self.timer_type != TimerType::NONE
    }

    pub fn update_timer(&mut self, delta: usize) {
        if self.timer_remained_ns == 0 {
            return;
        }
        if self.timer_remained_ns > delta {
            self.timer_remained_ns -= delta;
        }
    }
}
