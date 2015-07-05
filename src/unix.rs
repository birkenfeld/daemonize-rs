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
use libc::funcs::posix88::unistd;
use std::os::raw::c_int;
use std::os::unix::raw::{ gid_t, uid_t, mode_t };

mod ffi {
    use std::os::unix::raw::mode_t;
    extern {
        pub fn umask(mode: mode_t) -> mode_t;
    }
}

pub fn fork() -> io::Result<bool> {
    let ret = unsafe { unistd::fork() };
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
    let ret = unsafe { unistd::setsid() };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn close(fd: c_int) -> io::Result<()> {
    let ret = unsafe { unistd::close(fd) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn geteuid() -> uid_t {
    unsafe { unistd::geteuid() }
}

pub fn setgid(gid: gid_t) -> io::Result<()> {
    let ret = unsafe { unistd::setgid(gid) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn setuid(uid: uid_t) -> io::Result<()> {
    let ret = unsafe { unistd::setuid(uid) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}