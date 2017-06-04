use packets::{Packet, PacketType};
use types::User;

use ansi_term::Colour::{Yellow, Green};
use sha1::Sha1;
use toml;

use std::fs::File;
use std::env::temp_dir;
use std::process::exit;
use std::io::Write;

pub fn login(name: &str, passwd: &str)
{
	println!("  {}",
		Yellow.bold().paint("logging in")
	);

	let mut hash = Sha1::new();
	let bytes: Vec<u8> = passwd.bytes().collect();
	hash.update(&bytes);
	let hash = hash.digest().to_string();
	let res = Packet::login(name, &hash)
		.send();

	match res.ptype
	{
		PacketType::Error =>
		{
			println!("  error occured: {}", res.name.unwrap());
			exit(-1);
		},
		PacketType::Login =>
		{
			let mut f = match File::create(
				&{let mut temp = temp_dir(); temp.push("pebble_usr"); temp}
			)
			{
				Ok(f) => f,
				Err(_) =>
				{
					println!("  error: failed to create login file");
					exit(-1);
				}
			};

			let _ = write!(f, "{}",
				toml::to_string(&User
					{
						name: name.to_string(),
						hash: hash,
					}
				).unwrap()
			);

			println!("  {} as {}",
				Yellow.bold().paint("logged in"),
				Green.bold().paint(name)
			);
		},
		_ => {},
	}
}
