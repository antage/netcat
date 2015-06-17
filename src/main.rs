extern crate getopts;

use std::io::Write;

macro_rules! println_stderr {
	($fmt: expr) => (let _ = write!(&mut std::io::stderr(), concat!($fmt, "\n")).unwrap());
	($fmt: expr, $($arg: tt)*) => (let _ = write!(&mut std::io::stderr(), concat!($fmt, "\n"), $($arg)*));
}

macro_rules! try_or_exit {
	($e: expr, $msg_prefix: expr) => {
		match $e {
			Ok(result) => result,
			Err(e) => {
				println_stderr!("{}: {}", $msg_prefix, e);
				std::process::exit(1);
			}
		}
	}
}

fn usage(program: &str, opts: getopts::Options) {
	let brief = format!("Usage: {} [options]", program);
	println_stderr!("{}", opts.usage(&brief));
}

fn listen_mode(port: u16) {
	println_stderr!("Listen mode at TCP-port {}.", port);
	let listener = try_or_exit!(std::net::TcpListener::bind(("0.0.0.0", port)), "Can't bind to TCP-port");
	let (mut stream, _) = try_or_exit!(listener.accept(), "Can't accept connection");
	let total_bytes = try_or_exit!(std::io::copy(&mut stream, &mut std::io::stdout()), "Copy error");
	println_stderr!("{} byte(s) was received.", total_bytes);
}

fn send_mode(hostname: &str, port: u16) {
	println_stderr!("Send mode to '{}' hostname at TCP-port {}.", hostname, port);
	let mut stream = try_or_exit!(std::net::TcpStream::connect((hostname, port)), "Can't establish connection");
	let total_bytes = try_or_exit!(std::io::copy(&mut std::io::stdin(), &mut stream), "Copy error");
	println_stderr!("{} byte(s) was sent.", total_bytes);
}

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	let program = &args[0];

	let mut opts = getopts::Options::new();
	opts.optflag("h", "help", "print usage info");
	opts.optopt("l", "", "listen mode on specified TCP-port", "PORT");

	let matches = try_or_exit!(opts.parse(&args[1..]), "ERROR");
	if matches.opt_present("h") {
		usage(program, opts);
		std::process::exit(0);
	}

	if matches.opt_present("l") {
		let listen_port: u16 = match matches.opt_str("l") {
			Some(port_str) => try_or_exit!(port_str.parse(), "Can't parse TCP-port"),
			None => {
				println_stderr!("You should specify TCP-port for listen mode.");
				std::process::exit(1);
			}
		};

		listen_mode(listen_port);
	} else {
		if matches.free.len() == 0  && !matches.opt_present("h") {
			usage(&program, opts);
			std::process::exit(0);
		} else if matches.free.len() < 2 {
			println_stderr!("You should specify hostname and TCP-port.");
			std::process::exit(1);
		} else {
			let hostname = &matches.free[0];
			let port: u16 = try_or_exit!(matches.free[1].parse(), "Can't pare TCP-port");
			send_mode(hostname, port);
		}
	}
}
