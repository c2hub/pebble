use packets::{Packet, PacketType};

use ansi_term::Colour::{Yellow, Green, Red};
use std::process::exit;

pub fn find(name: &str)
{
	println!("  {} pebble [{}]",
		Yellow.bold().paint("find"),
		Green.bold().paint(name)
	);

	let res = Packet::find(name, "*").send();

	match res.ptype
	{
		PacketType::Error =>
		{
			println!("  error occured: {}", res.name.unwrap());
			exit(-1);
		},
		PacketType::Find =>
		{
			let data = res.data.unwrap();
			if data != "none"
			{
				println!("  {} [{}] version {}",
					Yellow.bold().paint("found"),
					Green.bold().paint(name),
					data
				);
			}
			else
			{
				println!("  {}",
					Red.bold().paint("could not be found")
				);
			}
		},
		_ => unreachable!(),
	}
}
