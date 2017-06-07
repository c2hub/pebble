use packets::{Packet, PacketType};
use errors::*;

use ansi_term::Colour::{Yellow, Green};
use sha1::Sha1;

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
		PacketType::Error => fail1("packet -> {}", res.name.unwrap(), 72),
		PacketType::Register =>
		{
			println!("  {}",
				Yellow.bold().paint("registration successful")
			);
		},
		_ => {},
	}
}
