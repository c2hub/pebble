use errors::{fail, fail1};
use packets::Packet;
use types::User;

use ansi_term::Colour::{Green, Yellow};
use sha1::Sha1;
use toml;

use std::env::temp_dir;
use std::fs::File;
use std::io::Write;

pub fn login(name: &str, passwd: &str) {
	println!("  {}", Yellow.bold().paint("logging in"));

	let mut hash = Sha1::new();
	let bytes: Vec<u8> = passwd.bytes().collect();
	hash.update(&bytes);
	let hash = hash.digest().to_string();
	let res = Packet::login(name, &hash).send();

	match res {
		Packet::Error { msg } => fail1("packet -> {}", msg, 42),
		Packet::Login { .. } => {
			let mut f = match File::create(&{
				let mut temp = temp_dir();
				temp.push("pebble_usr");
				temp
			}) {
				Ok(f) => f,
				Err(_) => fail("failed to create login file", 43),
			};

			let _ = write!(
				f,
				"{}",
				toml::to_string(&User {
					name: name.to_string(),
					hash: hash,
				}).unwrap()
			);

			println!(
				"  {} as {}",
				Yellow.bold().paint("logged in"),
				Green.bold().paint(name)
			);
		}
		_ => {}
	}
}
