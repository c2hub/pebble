use config::Config;
use errors::fail;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use recipe_reader::Recipe;
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn install() {
	let mut recipe = Recipe::new();

	if Recipe::find() != None {
		recipe.read_errors(true);
		if !recipe.ok {
			fail("failed to read recipe, exiting", 34);
		}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	} else {
		fail("no recipe found in path", 35);
	}

	if !Path::new("pebble.toml").exists() {
		fail("not a valid pebble, missing pebble.toml", 36);
	}

	let cfg = match Config::read() {
		Ok(c) => c,
		Err(_) => fail("failed to parse pebble.toml", 37),
	};

	let name = match recipe.path.parent().unwrap().file_stem() {
		Some(n) => match n.to_str() {
			Some(s) => s,
			None => fail("could not read name string", 38),
		},
		None => fail("invalid project path", 39),
	};

	if let Some(ref bcfg) = cfg.build {
		if let Some(ref install) = bcfg.install {
			println!(
				"  {} [{}]",
				Yellow.bold().paint("installing"),
				Green.bold().paint(name)
			);

			for cmd_str in install {
				println!("  {} '{}'", Yellow.bold().paint("executing"), &cmd_str);
				let name = cmd_str.split_whitespace().collect::<Vec<&str>>()[0];
				let args: Vec<&str> = cmd_str.split_whitespace().skip(1).collect();
				let cmd = Command::new(name)
					.args(args)
					.stdout(Stdio::inherit())
					.stderr(Stdio::inherit())
					.output()
					.expect("  error: failed to run command");
				if !cmd.status.success() {
					println!(
						"  {}: commmand '{}' returned non-zero exit code",
						Red.bold().paint("warning"),
						Blue.bold().paint(cmd_str.clone())
					);
				}
			}
		} else {
			fail("can't install due to missing install instructions", 40)
		}
	} else {
		fail("can't install due to missing build section", 41)
	}
}
