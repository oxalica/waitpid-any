use std::io::{Error, ErrorKind, Result};
use std::mem::MaybeUninit;
use std::time::Duration;
use std::fmt;

use libc::{wait_for_objects, wait_for_objects_etc, object_wait_info, get_team_info, team_info};
use libc::{B_OBJECT_TYPE_THREAD, B_EVENT_INVALID, B_BAD_TEAM_ID, B_RELATIVE_TIMEOUT};

pub struct WaitHandle(object_wait_info);

impl fmt::Debug for WaitHandle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("object_wait_info")
			.field("object", &self.0.object)
			.field("type_", &self.0.type_)
			.field("events", &self.0.events)
			.finish()
	}
}

pub fn open(pid: i32) -> Result<WaitHandle> {
	// `wait_for_objects[_etc]` can't discern between non-existing and exited teams,
	// so first check for a valid team with `get_team_info`
	let mut buf = MaybeUninit::<team_info>::zeroed();
	let ret = unsafe { get_team_info(pid, buf.as_mut_ptr()) };
	if ret == B_BAD_TEAM_ID {
		return Err(Error::new(ErrorKind::InvalidInput, format!("invalid PID {pid}")));
	}
	
	let wait_info = object_wait_info {
		object: pid,
		type_: B_OBJECT_TYPE_THREAD,
		events: B_EVENT_INVALID,
	};
	
	Ok(WaitHandle(wait_info))
}

pub fn wait(handle: &mut WaitHandle, timeout: Option<Duration>) -> Result<Option<()>> {
	let ret = match timeout {
		None => unsafe { wait_for_objects(&raw mut handle.0, 1) },
		Some(duration) => {
			let timeout = duration.as_secs() * 1_000_000 + u64::from(duration.subsec_micros());
			unsafe { wait_for_objects_etc(&raw mut handle.0, 1, B_RELATIVE_TIMEOUT, timeout.cast_signed()) }
		},
	};
	
	match (ret, handle.0.events) {
		(1, B_EVENT_INVALID) => Ok(Some(())),
		_ => Ok(None),
	}
}
