use errors::{fail, fail1};
use util::PACKET_BYTES;
use commands::package;
use packets::Packet;
use config::Config;
use types::User;

use ansi_term::Colour::{Green, Yellow, Red};
use version_compare::Version;
use recipe_reader::Recipe;
use pbr::ProgressBar;
use toml;

use std::env::set_current_dir;
use std::net::UdpSocket;
use std::time::Duration;
use std::env::temp_dir;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn upload()
{
	let mut bytes = package();

	let mut recipe = Recipe::new();

	if Recipe::find() != None
	{
		recipe.read_errors(true);
		if !recipe.ok
			{fail("failed to read recipe, exiting", 102);}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	}
	else
		{fail("no recipe found in path", 103);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 104);}

	let cfg = match Config::read()
	{
		Ok(c) => c,
		Err(_) => fail("failed to parse pebble.toml", 105)
	};

	let mut f = match File::open(
		&{let mut temp = temp_dir(); temp.push("pebble_usr"); temp}
	)
	{
		Ok(f) => f,
		Err(_) => fail("failed to open login file, are you logged in", 106)
	};

	let user: User = match toml::from_str(&{
		let mut s = String::new();
		if f.read_to_string(&mut s).is_err()
			{fail("failed to read login file", 107);}
		s
	})
	{
		Ok(u) => u,
		Err(_) => fail("failed to parse login file, relogin", 108)
	};

	if !Version::from(&cfg.pebble.version).is_some()
		{fail("version string is not a valid version", 109);}

	println!("  {} pebble [{}] version {}",
		Yellow.bold().paint("uploading"),
		Green.bold().paint(cfg.pebble.name.as_ref()),
		Red.bold().paint(cfg.pebble.version.as_ref()),
	);

	let len = bytes.len() as u32;
	let res = Packet::upload(
		&user.name,
		&user.hash,
		// All packets will carry PACKET_BYTES bytes. If the amount of bytes isn't divisible
		// by PACKET_BYTES, then the last packet will carry all the remaining data.
		// The amount of packets is either len / PACKET_BYTES or len / PACKET_BYTES + 1
		// if it's with a remainder
		(len - (len % PACKET_BYTES)) / PACKET_BYTES + 
		(if len % PACKET_BYTES != 0 {1} else {0}),
		&cfg.pebble.name,
		&cfg.pebble.version,
	).send();

	match res
	{
		Packet::Error { msg } => fail1("packet -> {}", msg, 110),
		Packet::Upload { parts, .. } =>
		{
			// just for clarity
			let port = parts;
			let parts = 
			(len - (len % PACKET_BYTES)) / PACKET_BYTES + 
			(if len % PACKET_BYTES != 0 {1} else {0});
			
			let socket = match UdpSocket::bind("0.0.0.0:0")
			{
				Ok(s) => s,
				Err(_) => fail("failed to bind to socket", 1101),
			};

			if socket.connect(format!("magnusi.tech:{}", port)).is_err()
				{fail("failed to connect to remote host. are you connected to the internet?", 5132)}

			socket.set_write_timeout(Some(Duration::from_millis(600)))
				.expect("failed to set timeout duration for socket");

			println!("  {} established",
				Yellow.bold().paint("connection")
			);

			match parts
			{
				0 => fail("the data vector is empty", 865),
				1 =>
				{
					match Packet::transfer(1, bytes).send_to(&socket)
					{
						Packet::Transfer { .. } =>
						{
							println!("  {} successful",
								Yellow.bold().paint("transfer")
							);
						},
						Packet::Error { msg } => fail1("packet -> {}", msg, 110),
						_ => unreachable!(),
					}
				},
				_ =>
				{
					let mut current_part = 1;
					let mut pb = ProgressBar::new(parts as u64);
					let mut current_bytes: Vec<u8> = if PACKET_BYTES as usize > bytes.len()
						{bytes.drain(..PACKET_BYTES as usize).collect()}
					else
						{bytes.drain(..).collect()};

					loop
					{
						match Packet::transfer(current_part, current_bytes.clone())
							.send_to(&socket)
						{
							Packet::Transfer { part, .. } =>
							{
								if part == parts + 1
								{
									pb.inc();
									pb.finish_print("done");
									break;
								}
								else if part == current_part
								{
									continue;
								}
								else
								{
									current_part += 1;
									current_bytes = if PACKET_BYTES as usize > bytes.len()
										{bytes.drain(..PACKET_BYTES as usize).collect()}
									else
										{bytes.drain(..).collect()};
									pb.inc();
								}
							},
							Packet::Error { msg } => fail1("packet -> {}", msg, 110),
							_ => unreachable!(),
						}
					}

					println!("  {} successful",
						Yellow.bold().paint("transfer")
					);
				}
			}
		}
		_ => unreachable!(),
	}
}
