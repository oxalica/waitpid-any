use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

use rustix::event::kqueue::{kevent, kqueue, Event, EventFilter, EventFlags, ProcessEvents};
use rustix::io::retry_on_intr;
use rustix::process::Pid;

// KQueue only emits the event once. So we will store the exited state to make `wait` returns
// `Ok(Some(()))` immediately after the event fires.
#[derive(Debug)]
pub enum WaitHandle {
    KQueue(rustix::fd::OwnedFd),
    Exited,
}

pub fn open(pid: i32) -> Result<WaitHandle> {
    let pid = Pid::from_raw(pid)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, format!("invalid PID {pid}")))?;
    let kqueue = kqueue()?;
    let event = Event::new(
        EventFilter::Proc {
            pid,
            flags: ProcessEvents::EXIT,
        },
        EventFlags::ADD,
        0,
    );
    unsafe {
        kevent(&kqueue, &[event], &mut Vec::new(), None)?;
    }
    Ok(WaitHandle::KQueue(kqueue))
}

pub fn wait(handle: &mut WaitHandle, timeout: Option<Duration>) -> Result<Option<()>> {
    let kqueue = match handle {
        WaitHandle::KQueue(kqueue) => kqueue,
        WaitHandle::Exited => return Ok(Some(())),
    };

    let mut buf = Vec::with_capacity(1);
    let ret = unsafe { retry_on_intr(|| kevent(&kqueue, &[], &mut buf, timeout)) };
    match ret {
        // Timeout.
        Ok(0) => Ok(None),
        // Exited.
        Ok(_) => {
            // Fuse the handle.
            *handle = WaitHandle::Exited;
            Ok(Some(()))
        }
        // Something went wrong. Events should not be consumed here.
        Err(err) => Err(err.into()),
    }
}
