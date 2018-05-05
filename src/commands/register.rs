use errors::fail1;
use packets::Packet;

use ansi_term::Colour::{Green, Yellow};
use sha1::Sha1;

pub fn register(name: &str, passwd: &str) {
	println!(
		"  {} user {}",
		Yellow.bold().paint("registering"),
		Green.bold().paint(name)
	);

	let mut hash = Sha1::new();
	let bytes: Vec<u8> = passwd.bytes().collect();
	hash.update(&bytes);
	let res = Packet::register(name, &hash.digest().to_string()).send();

	match res {
		Packet::Error { msg } => fail1("packet -> {}", msg, 72),
		Packet::Register { .. } => {
			println!("  {}", Yellow.bold().paint("registration successful"));
		}
		_ => {}
	}
}
