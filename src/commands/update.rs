use ansi_term::Colour::{Yellow, Green};
use hyper::client::Client;
use recipe_reader::Recipe;
use zip::ZipArchive;

use std::path::Path;
use std::io::{Read, Write};
use std::env::set_current_dir;
use std::fs::{File, create_dir, read_dir};

use packets::Packet;
use errors::{fail, fail1};
use commands::find::find_ver;
use config::{Dependency, Config};

pub fn update()
{
	let mut _recipe = Recipe::new();

	if Recipe::find() != None
		{_recipe.read(); let _ = set_current_dir(Path::new(&_recipe.path.parent().unwrap()));}
	else
		{fail("no recipe found in path",9);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 10);}

	let cfg = match Config::read()
	{
		Ok(c) => c,
		Err(_) => fail("failed to read config", 6451)
	};

	println!("  {} http://magnusi.tech/static/pebbles/data for pebbles",
		Yellow.bold().paint("scanning")
	);

	let index = Packet::update("hello").send();

	match index
	{
		Packet::Update { .. } =>
		{
			if let Some(deps) = cfg.dependencies
			{
				fetch_deps(deps);
				unpack_deps();
			} else { println!("fail?"); return;}
		},
		Packet::Error { msg } => fail1("packet -> {}", msg, 7452),
		_ => unreachable!()
	}
}

pub fn fetch_deps(deps: Vec<Dependency>)
{
	for dep in deps
	{
		println!("  {} for [{}] version {}",
			Yellow.bold().paint("searching"),
			Green.bold().paint(dep.name.as_ref()),
			&dep.version,
		);
		if find_ver(&format!("lib{}", &dep.name), &dep.version)
		{
			let last_ver = match Packet::find(&format!("lib{}", &dep.name), &dep.version)
				.send()
			{
				Packet::Find { version, .. } => version,
				Packet::Error { msg } => fail1("packet -> {}", msg, 110),
				_ => unreachable!(),
			};

			println!("  {} [{}] {} ",
				Yellow.bold().paint("downloading"),
				Green.bold().paint(dep.name.as_ref()),
				&last_ver
			);

			let mut lib = match Client::new()
				.get(
					&format!(
						"http://magnusi.tech/static/pebbles/data/lib{}/{}/libpackage.zip",
						&dep.name,
						&last_ver
					)
				)
				.send()
			{
				Ok(res) => res,
				Err(_) => fail("failed to acquire pebble, are you connected to the internet?", 4521)
			};

			let mut bytes = Vec::new();

			if lib.read_to_end(&mut bytes).is_err()
				{fail("failed to read libpackage.zip", 4512);}

			if create_dir(Path::new("libs")).is_err() && !Path::new("libs").exists()
			{
				fail("failed to create libs directory", 4458);
			}

			if let Ok(mut pkg) = File::create(&format!("libs/lib{}.zip", &dep.name))
			{
				if pkg.write_all(&bytes).is_err()
				{
					fail("failed to write zip file", 8451);
				}
			}
		}
		else
		{
			fail("one or more pebbles not found", 8456);
		}
	}
}

pub fn unpack_deps()
{
	for f in match read_dir(Path::new("libs"))
		{ Ok(r) => r, _ => fail("failed to open source directory", 451) }
	{
		let file = match f
		{
			Ok(fl) => fl,
			Err(_) => fail("failed to read a directory entry in libs", 8485)
		};

		let mut archive = match ZipArchive::new(File::open(file.path()).unwrap())
		{
			Ok(z) => z,
			Err(e) => { println!("{}", &e); continue;},
		};

		let dname = file.path()
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.replace("lib", "");

		println!(" {} [{}]",
			Yellow.bold().paint("unpacking"),
			Green.bold().paint(dname.as_ref())
		);

		if create_dir(&dname).is_err()
			{fail("couldn't create library source dir", 1258);}

		let _ = set_current_dir(&dname);

		for i in 0..archive.len()
		{
			if let Ok(mut f) = archive.by_index(i)
			{
				if let Ok(mut cf) = File::create(f.name())
				{
					let mut bytes = Vec::new();
					if f.read_to_end(&mut bytes).is_err()
						{fail("couldn't read file in zip", 4515);}
					if cf.write_all(&bytes).is_err()
						{fail("couldn't write file from zip", 4487);}
				}
			} else { fail("failed to read zip entry", 4613); }
		}

		let _ = set_current_dir("..");
	}
}
