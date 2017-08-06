use packets::{Packet, PacketType};
use errors::fail1;

use ansi_term::Colour::{Yellow, Green, Red};

//TODO make DRY-ish

pub fn find(name: &str)
{
	println!("  {} pebble [{}]",
		Yellow.bold().paint("find"),
		Green.bold().paint(name)
	);

	let res = Packet::find(name, "*").send();

	match res.ptype
	{
		PacketType::Error => fail1("packet -> {}", res.name.unwrap(), 18),
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

pub fn find_ver(name: &str, version: &str) -> bool
{
	println!("  {} pebble [{}]",
		Yellow.bold().paint("find"),
		Green.bold().paint(name)
	);

	let res = Packet::find(name, version).send();

	match res.ptype
	{
		PacketType::Error => fail1("packet -> {}", res.name.unwrap(), 18),
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
				true
			}
			else
			{
				println!("  {}",
					Red.bold().paint("could not be found")
				);
				false
			}
		},
		_ => unreachable!(),
	}
}
