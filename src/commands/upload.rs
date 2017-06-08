use packets::{Packet, PacketType};
use commands::package;
use config::Config;
use errors::fail;
use types::User;

use ansi_term::Colour::{Green, Yellow, Red};
use version_compare::Version;
use recipe_reader::Recipe;
use toml;

use std::env::set_current_dir;
use std::env::temp_dir;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn upload()
{
	let bytes = package();

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

	let res = Packet::upload(
		&user.name,
		&user.hash,
		bytes,
		&cfg.pebble.name,
		&cfg.pebble.version,
	).send();

	match res.ptype
	{
		PacketType::Error => fail1("packet -> {}", res.name.unwrap(), 110),
		PacketType::Upload =>
		{
			println!("  {} successful",
				Yellow.bold().paint("upload")
			);
		}
		_ => unreachable!(),
	}
}
