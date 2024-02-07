// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.
//use failure::{Backtrace, Context, Fail};
use std::io;
use thiserror::Error;

/// Possible errors encountered while creating daemon
#[derive(Error, Debug)]
pub enum Error {
    /// Call to pipe failed
    #[error("Call to pipe failed: {0}")]
    Pipe(io::Error),

    /// Couldn't fork daemon process
    #[error("Couldn't fork daemon process: {0}")]
    Fork(io::Error),

    /// Couldn't redirect stdio streams
    #[error("Couldn't redirect stdio streams: {0}")]
    Dup2(io::Error),

    /// Unable to create new session
    #[error("Unable to create new session: {0}")]
    DetachSession(io::Error),

    /// Unable to change directory
    #[error("Unable to change directory")]
    ChangeDirectory,

    /// pid_file option contains NUL
    #[error("pid_file option contains NUL")]
    PathContainsNul,

    /// Unable to open pid file
    #[error("Unable to open pid file, {0}")]
    OpenPidfile(io::Error),

    /// Unable to write self pid to pid file
    #[error("Unable to write self pid to pid file {0}")]
    WritePid(io::Error),

    /// failed to register pipe fd's with mio
    #[error("Unable to register pipe with mio: {0}")]
    RegisterationError(io::Error),

    /// splice returned an error
    #[error("Unable to splice streams: {0}")]
    SpliceError(io::Error),

    /// couldn't get the pending datasize from ioctl
    #[error("Unable to fetch pending datasize from ioctl: {0}")]
    Ioctl(io::Error),

    /// fnctl operations failed
    #[error("{0}")]
    Fnctl(io::Error),

    /// attempted to daemonize on an unsupported platform
    #[error("Daemonize on the current platform is not supported")]
    UnsupportedPlatform,
}
