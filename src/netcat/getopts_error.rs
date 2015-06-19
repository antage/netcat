extern crate getopts;

use std;

#[derive(Debug)]
pub struct Fail(pub getopts::Fail);

impl std::fmt::Display for Fail {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let Fail(ref fail) = *self;
		fail.fmt(f)
	}
}

impl std::error::Error for Fail {
	fn description(&self) -> &str {
		let Fail(ref fail) = *self;
		match *fail {
			getopts::Fail::ArgumentMissing(ref s) => s,
			getopts::Fail::UnrecognizedOption(ref s) => s,
			getopts::Fail::OptionMissing(ref s) => s,
			getopts::Fail::OptionDuplicated(ref s) => s,
			getopts::Fail::UnexpectedArgument(ref s) => s
		}
	}
}

impl std::convert::From<getopts::Fail> for Fail {
	fn from(err: getopts::Fail) -> Fail {
		Fail(err)
	}
}
