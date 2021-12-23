#![cfg(unix)]
use std::env::args_os;
use std::io::{self, ErrorKind};
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn main() -> io::Result<()> {
	let path = args_os().nth(1).ok_or(ErrorKind::InvalidInput)?;

	let mut sock = UnixStream::connect(&path)?;
	sock.set_read_timeout(Some(Duration::from_secs(0))).ok();
	sock.set_write_timeout(Some(Duration::from_secs(2))).ok();

	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	io::copy(&mut stdin, &mut sock).map(|_| ())
}
