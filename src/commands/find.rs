use errors::fail1;
use packets::Packet;

use ansi_term::Colour::{Green, Red, Yellow};

//TODO make DRY-ish

pub fn find(name: &str) {
	println!(
		"  {} pebble [{}]",
		Yellow.bold().paint("find"),
		Green.bold().paint(name)
	);

	let res = Packet::find(name, "*").send();

	match res {
		Packet::Error { msg } => fail1("packet -> {}", msg, 18),
		Packet::Find { name, version } => {
			if version != "none" {
				println!(
					"  {} [{}] version {}",
					Yellow.bold().paint("found"),
					Green.bold().paint(name),
					version
				);
			} else {
				println!("  {}", Red.bold().paint("could not be found"));
			}
		}
		_ => unreachable!(),
	}
}

pub fn find_ver(name: &str, version: &str) -> bool {
	println!(
		"  {} pebble [{}]",
		Yellow.bold().paint("find"),
		Green.bold().paint(name)
	);

	let res = Packet::find(name, version).send();

	match res {
		Packet::Error { msg } => fail1("packet -> {}", msg, 18),
		Packet::Find { name, version } => {
			if version != "none" {
				println!(
					"  {} [{}] version {}",
					Yellow.bold().paint("found"),
					Green.bold().paint(name),
					version
				);
				true
			} else {
				println!("  {}", Red.bold().paint("could not be found"));
				false
			}
		}
		_ => unreachable!(),
	}
}
