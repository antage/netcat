extern crate getopts;

use std;

use super::getopts_error;

#[derive(Debug)]
pub enum Error {
	GetoptsFail(getopts_error::Fail),
	PortIsAbsent(String),
	HostnameAndPortAreAbsent(String),
	PortParse(std::num::ParseIntError),
	IO(std::io::Error),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::GetoptsFail(ref err) => write!(f, "Command-line arguments parsing error: {}", err),
			Error::PortIsAbsent(ref err) => write!(f, "Listen mode requires port number after '-l' flag: {}", err),
			Error::HostnameAndPortAreAbsent(ref err) => write!(f, "Send mode requires hostname and port: {}", err),
			Error::PortParse(ref err) => write!(f, "Can't parse TCP-port number: {}", err),
			Error::IO(ref err) => write!(f, "I/O error: {}", err)
		}
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::GetoptsFail(ref err) => err.description(),
			Error::PortIsAbsent(ref err) => err,
			Error::HostnameAndPortAreAbsent(ref err) => err,
			Error::PortParse(ref err) => std::error::Error::description(err),
			Error::IO(ref err) => err.description()
		}
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match *self {
			Error::GetoptsFail(ref err) => Some(err),
			Error::PortIsAbsent(_) => None,
			Error::HostnameAndPortAreAbsent(_) => None,
			Error::PortParse(ref err) => Some(err),
			Error::IO(ref err) => Some(err)
		}
	}
}

impl std::convert::From<getopts::Fail> for Error {
	fn from(err: getopts::Fail) -> Error {
		Error::GetoptsFail(getopts_error::Fail(err))
	}
}

impl std::convert::From<std::num::ParseIntError> for Error {
	fn from(err: std::num::ParseIntError) -> Error {
		Error::PortParse(err)
	}
}

impl std::convert::From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Error {
		Error::IO(err)
	}
}
