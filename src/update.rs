use ansi_term::Colour::Yellow;
use hyper::client::Client;
use std::process::exit;
use std::io::Read;

pub fn update()
{
	println!("  {} http://magnusi.tech/static/pebbles/ for pebbles",
		Yellow.bold().paint("scanning")
	);

	let mut index = match Client::new().get("http://magnusi.tech/static/index").send()
	{
		Ok(res) => res,
		Err(_) =>
		{
			println!("  error: failed to acquire pebble index, are you connected to the internet?");
			exit(-1);
		}
	};

	println!("{}", { let mut s = String::new(); let _ = index.read_to_string(&mut s); s})
}
