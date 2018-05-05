use commands::build;
use config::Config;
use errors::{fail, fail1};
use packets::Packet;
use types::User;

use ansi_term::Colour::{Green, Red, Yellow};
use recipe_reader::Recipe;
use toml;
use version_compare::Version;
use zip;

use std::env::{set_current_dir, temp_dir};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn publish() {
	build();

	let mut recipe = Recipe::new();

	if Recipe::find() != None {
		recipe.read_errors(true);
		if !recipe.ok {
			fail("failed to read recipe, exiting", 111);
		}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	} else {
		fail("no recipe found in path", 112);
	}

	if !Path::new("pebble.toml").exists() {
		fail("not a valid pebble, missing pebble.toml", 113);
	}

	let cfg = match Config::read() {
		Ok(c) => c,
		Err(_) => fail("failed to read pebble manifest", 114),
	};

	let name = match recipe.path.parent().unwrap().file_stem() {
		Some(n) => match n.to_str() {
			Some(s) => s,
			None => fail("could not read name string", 115),
		},
		None => fail("invalid project path", 116),
	};

	println!(
		"  {} pebble [{}]",
		Yellow.bold().paint("publishing"),
		Green.bold().paint(name)
	);

	let mut zip_f = match File::create("libpackage.zip") {
		Ok(f) => f,
		Err(_) => fail("failed to create libpackage file", 117),
	};

	let mut zip = zip::ZipWriter::new(&mut zip_f);

	if let Some(libcfg) = cfg.lib {
		for fil in libcfg.claim {
			let mut f = match File::open(&fil) {
				Ok(f) => f,
				Err(_) => fail("failed to open directory entry for reading", 118),
			};

			if zip.start_file(fil, zip::write::FileOptions::default())
				.is_err()
			{
				fail("failed to start a file in zip", 119);
			}

			let mut content = String::new();

			if f.read_to_string(&mut content).is_ok() && zip.write(&{
				let v: Vec<u8> = content.bytes().collect();
				v
			}).is_err()
			{
				fail("failed to write file to zip", 120);
			}
		}

		if let Some(extra) = libcfg.extra {
			for fil in extra {
				let mut f = match File::open(&fil) {
					Ok(f) => f,
					Err(_) => fail1("failed to open '{}' for reading", &fil, 121),
				};

				if zip.start_file(fil, zip::write::FileOptions::default())
					.is_err()
				{
					fail("failed to start a file in zip", 122);
				}

				let mut content = String::new();

				if f.read_to_string(&mut content).is_ok() && zip.write(&{
					let v: Vec<u8> = content.bytes().collect();
					v
				}).is_err()
				{
					fail("failed to write file to zip", 123);
				}
			}
		}

		if zip.start_file("pebble.toml", zip::write::FileOptions::default())
			.is_err()
		{
			fail("failed to start a file in zip", 122);
		}

		let mut f = match File::open("pebble.toml") {
			Ok(f) => f,
			Err(_) => fail("failed to open pebble.toml for reading", 121),
		};
		let mut content = String::new();

		if f.read_to_string(&mut content).is_ok() && zip.write(&{
			let v: Vec<u8> = content.bytes().collect();
			v
		}).is_err()
		{
			fail("failed to write file to zip", 123);
		}
	} else {
		fail("missing libcfg", 124);
	}

	if zip.finish().is_err() {
		fail("failed to finish writing zip", 125);
	}

	let mut zip_f = match File::open("libpackage.zip") {
		Ok(f) => f,
		Err(_) => fail("failed to open zip file for reading", 126),
	};

	let mut bytes = Vec::new();
	if zip_f.read_to_end(&mut bytes).is_err() {
		fail("failed to read zip file", 127);
	}

	let mut f = match File::open(&{
		let mut temp = temp_dir();
		temp.push("pebble_usr");
		temp
	}) {
		Ok(f) => f,
		Err(_) => fail("failed to open login file, are you logged in", 106),
	};

	let user: User = match toml::from_str(&{
		let mut s = String::new();
		if f.read_to_string(&mut s).is_err() {
			fail("failed to read login file", 128);
		}
		s
	}) {
		Ok(u) => u,
		Err(_) => fail("failed to parse login file, relogin", 129),
	};

	if !Version::from(&cfg.pebble.version).is_some() {
		fail("version string is not a valid version", 130);
	}

	println!(
		"  {} pebble [{}] version {}",
		Yellow.bold().paint("uploading"),
		Green.bold().paint(cfg.pebble.name.as_ref()),
		Red.bold().paint(cfg.pebble.version.as_ref()),
	);

	let res = Packet::publish(
		&user.name,
		&user.hash,
		bytes,
		&cfg.pebble.name,
		&cfg.pebble.version,
	).send();

	match res {
		Packet::Error { msg } => fail1("packet -> {}", msg, 131),
		Packet::Publish { .. } => {
			println!("  {} successful", Yellow.bold().paint("upload"));
		}
		_ => unreachable!(),
	}
}
