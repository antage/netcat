mod netcat;

use std::io::Write;
use std::process::exit;
use std::io::stderr;

fn main() {
	let result =
		match netcat::parse_args() {
			Ok(args) => match args {
				netcat::Args::Usage(program, opts) => {
					netcat::usage(&program, opts);
					Ok(())
				},
				netcat::Args::ListenMode(port) => netcat::listen_mode(port),
				netcat::Args::SendMode(ref hostname, port) => netcat::send_mode(hostname, port)
			},
			Err(err) => Err(err)
		};
	match result {
		Err(err) => {
			writeln!(&mut stderr(), "{}", err).unwrap();
			exit(1);
		},
		_ => ()
	}
}
