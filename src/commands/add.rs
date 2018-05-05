use ansi_term::Colour::{Green, Red, Yellow};
use recipe_reader::Recipe;
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

use errors::{fail, fail1};

pub fn add(filename: &str) {
	let mut recipe = Recipe::new();
	let mut path = filename.to_string();

	if Recipe::find() != None {
		recipe.read();
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	} else {
		fail("no recipe found in path", 9);
	}

	if !Path::new("pebble.toml").exists() {
		fail("not a valid pebble, missing pebble.toml", 10);
	}

	if Path::new(filename).exists() {
		for t in &mut recipe.targets {
			t.files.push(filename.to_string());

			println!(
				"  {} {} to [{}]",
				Yellow.bold().paint("added"),
				filename,
				Green.bold().paint(t.name.clone())
			);
		}
	} else if !Path::new(filename).exists() && Path::new(&("src/".to_string() + filename)).exists()
	{
		for t in &mut recipe.targets {
			t.files.push("src/".to_string() + filename);

			println!(
				"  {} src/{} to [{}]",
				Yellow.bold().paint("added"),
				filename,
				Green.bold().paint(t.name.clone())
			);
		}

		path = "src/".to_string() + filename;
	} else {
		fail1("'{}' not found", filename, 11);
	}

	let cmd = Command::new("git")
		.arg("add")
		.arg(&path)
		.output()
		.expect("  error: failed to run command");
	if !cmd.status.success() {
		println!(
			"  {} git returned a non-zero exit code",
			Red.bold().paint("error")
		);
	}

	recipe.write();
}
