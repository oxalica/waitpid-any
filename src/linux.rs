use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

use rustix::event::{poll, PollFd, PollFlags};
use rustix::io::retry_on_intr;
use rustix::process::{pidfd_open, Pid, PidfdFlags};

pub type WaitHandle = rustix::fd::OwnedFd;

pub fn open(pid: i32) -> Result<WaitHandle> {
    let pid = Pid::from_raw(pid)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, format!("invalid PID {pid}")))?;
    let pidfd = pidfd_open(pid, PidfdFlags::empty())?;
    Ok(pidfd)
}

pub fn wait(pidfd: &mut WaitHandle, timeout: Option<Duration>) -> Result<Option<()>> {
    let timeout = match timeout {
        Some(dur) => dur.as_millis().try_into().unwrap_or(i32::MAX),
        None => -1, // Infinite.
    };
    let mut fds = [PollFd::new(&pidfd, PollFlags::IN)];
    let ret = retry_on_intr(|| poll(&mut fds, timeout))?;
    if ret == 0 {
        return Ok(None);
    }
    Ok(Some(()))
}
