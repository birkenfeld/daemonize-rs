// -----------------------------------------------------------------------------
// A simple daemonizing implementation using libc bindings.
//
// This program is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc.,
// 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// Module authors:
//   Georg Brandl <georg.brandl@frm2.tum.de>
//
// -----------------------------------------------------------------------------

use std::io;
use libc;
use libc::{uid_t, gid_t, mode_t};
use std::os::raw::c_int;

mod ffi {
    use libc::mode_t;
    extern {
        pub fn umask(mode: mode_t) -> mode_t;
    }
}

pub fn fork() -> io::Result<bool> {
    let ret = unsafe { libc::fork() };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else if ret == 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn umask(mode: mode_t) -> mode_t {
    unsafe { ffi::umask(mode) }
}

pub fn setsid() -> io::Result<()> {
    let ret = unsafe { libc::setsid() };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn close(fd: c_int) -> io::Result<()> {
    let ret = unsafe { libc::close(fd) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn geteuid() -> uid_t {
    unsafe { libc::geteuid() }
}

pub fn setgid(gid: gid_t) -> io::Result<()> {
    let ret = unsafe { libc::setgid(gid) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn setuid(uid: uid_t) -> io::Result<()> {
    let ret = unsafe { libc::setuid(uid) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
