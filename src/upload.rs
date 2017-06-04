use packets::{Packet, PacketType};
use package::package;
use config::Config;
use types::User;

use toml;
use recipe_reader::*;
use version_compare::Version;
use ansi_term::Colour::{Green, Yellow, Red};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::env::temp_dir;
use std::env::set_current_dir;

pub fn upload()
{
	let bytes = package();

	let mut recipe = Recipe::new();

	if Recipe::find() != None
	{
		recipe.read_errors(true);
		if !recipe.ok
		{
			println!("  error: failed to read recipe, exiting");
			exit(-1);
		}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	}
	else
		{println!("  error: no recipe found in path"); exit(-1);}

	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}

	let cfg = match Config::read()
	{
		Ok(c) => c,
		Err(_) =>
		{
			println!("  error: failed to parse pebble.toml");
			exit(-1);
		}
	};

	let mut f = match File::open(
		&{let mut temp = temp_dir(); temp.push("pebble_usr"); temp}
	)
	{
		Ok(f) => f,
		Err(_) =>
		{
			println!("  error: failed to open login file, are you logged in?");
			exit(-1);
		}
	};

	let user: User = match toml::from_str(&{
		let mut s = String::new();
		if f.read_to_string(&mut s).is_err()
		{
			println!("  error: failed to read login file");
			exit(-1);
		}
		s
	})
	{
		Ok(u) => u,
		Err(_) =>
		{
			println!("  error: failed to parse login file, relogin.");
			exit(-1);
		}
	};

	if Version::from(&cfg.pebble.version).is_some()
	{
		println!("  error: version string is not a valid version");
		exit(-1);
	}

	println!("  {} pebble [{}] version {}",
		Yellow.bold().paint("uploading"),
		Green.bold().paint(cfg.pebble.name.as_ref()),
		Red.bold().paint(cfg.pebble.version.as_ref()),
	);

	let res = Packet::upload(
		&user.name,
		&user.hash,
		bytes,
		cfg.pebble.name.as_ref(),
		cfg.pebble.version.as_ref(),
	).send();

	match res.ptype
	{
		PacketType::Error =>
		{
			println!(" error occured: {}", res.name.unwrap());
			exit(-1);
		},
		PacketType::Upload =>
		{
			println!("  {} successful",
				Yellow.bold().paint("upload")
			);
		}
		_ => unreachable!(),
	}
}
