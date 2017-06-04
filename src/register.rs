use packets::{Packet, PacketType};

use ansi_term::Colour::{Yellow, Green};
use sha1::Sha1;
use std::process::exit;

pub fn register(name: &str, passwd: &str)
{
	println!("  {} user {}",
		Yellow.bold().paint("registering"),
		Green.bold().paint(name)
	);

	let mut hash = Sha1::new();
	let bytes: Vec<u8> = passwd.bytes().collect();
	hash.update(&bytes);
	let res = Packet::register(name, &hash.digest().to_string())
		.send();

	match res.ptype
	{
		PacketType::Error =>
		{
			println!("  error occured: {}", res.name.unwrap());
			exit(-1);
		},
		PacketType::Register =>
		{
			println!("  {}",
				Yellow.bold().paint("registration successful")
			);
		},
		_ => {},
	}
}
