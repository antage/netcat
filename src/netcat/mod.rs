extern crate getopts;

mod getopts_error;
pub mod error;

use std;

use self::error::Error;

pub enum Args {
	SendMode(String, u16),
	ListenMode(u16),
	Usage(String, getopts::Options),
}

pub fn usage(program: &str, opts: getopts::Options) {
	let brief = format!("Usage: {} [options]", program);
	println!("{}", opts.usage(&brief))
}

pub fn listen_mode(port: u16) -> Result<(), Error> {
	let listener = try!(std::net::TcpListener::bind(("0.0.0.0", port)));
	let (mut stream, _) = try!(listener.accept());
	try!(std::io::copy(&mut stream, &mut std::io::stdout()));
	Ok(())
}

pub fn send_mode(hostname: &str, port: u16) -> Result<(), Error> {
	let mut stream = try!(std::net::TcpStream::connect((hostname, port)));
	try!(std::io::copy(&mut std::io::stdin(), &mut stream));
	Ok(())
}

pub fn parse_args() -> Result<Args, Error> {
	let args = std::env::args().collect::<Vec<String>>();
	let program = &args[0];

	let mut opts = getopts::Options::new();
	opts.optflag("h", "help", "print usage info");
	opts.optopt("l", "", "listen mode on specified TCP-port", "PORT");

	let matches = try!(opts.parse(&args[1..]));
	if matches.opt_present("h") {
		return Ok(Args::Usage(program.clone(), opts));
	}

	if matches.opt_present("l") {
		let listen_port: u16 =
			try!(
				try!(
					matches
						.opt_str("l")
						.ok_or(Error::PortIsAbsent("You should specify TCP-port for listen mode."))
				)
					.parse()
			);

		Ok(Args::ListenMode(listen_port))
	} else {
		if matches.free.len() == 0  && !matches.opt_present("h") {
			Ok(Args::Usage(program.clone(), opts))
		} else if matches.free.len() < 2 {
			Err(Error::HostnameAndPortAreAbsent("You should specify hostname and TCP-port."))
		} else {
			let hostname = &matches.free[0];
			let port: u16 = try!(matches.free[1].parse());

			Ok(Args::SendMode(hostname.clone(), port))
		}
	}
}
