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

extern crate libc;
extern crate users;

mod unix;

use std::env;
use std::io::{self, Write};
use std::process;

use libc::{gid_t, uid_t, mode_t};
use users::os::unix::UserExt;


pub enum Acct {
    ByName(String),
    ById(i64),
}

pub struct DaemonSettings {
    pub user:  Option<Acct>,
    pub group: Option<Acct>,
    pub umask: Option<mode_t>,
}

pub fn daemonize(settings: DaemonSettings) -> io::Result<()> {
    // finish up standard in/out
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();

    // first fork
    if try!(unix::fork()) == false {
        process::exit(0);
    }

    // decouple from parent environment
    try!(env::set_current_dir("/"));
    unix::umask(0o002);
    try!(unix::setsid());

    // second fork
    if try!(unix::fork()) == false {
        process::exit(0);
    }

    // switch user/group IDs if we are root
    if unix::geteuid() == 0 {
        if let Some(acct) = settings.group {
            let group = match acct {
                Acct::ById(id)  => users::get_group_by_gid(id as gid_t),
                Acct::ByName(n) => users::get_group_by_name(&n),
            };
            match group {
                None => return Err(io::Error::from_raw_os_error(2)), // ENOENT
                Some(group) => try!(unix::setgid(group.gid())),
            }
        }
        if let Some(acct) = settings.user {
            let user = match acct {
                Acct::ById(id)  => users::get_user_by_uid(id as uid_t),
                Acct::ByName(n) => users::get_user_by_name(&n),
            };
            match user {
                None => return Err(io::Error::from_raw_os_error(2)),
                Some(user) => {
                    try!(unix::setuid(user.uid()));
                    env::set_var("HOME", user.home_dir());
                },
            }
        }
    }
    if let Some(umask) = settings.umask {
        unix::umask(umask);
    }

    // close standard fds
    try!(unix::close(0));
    try!(unix::close(1));
    try!(unix::close(2));

    Ok(())
}
