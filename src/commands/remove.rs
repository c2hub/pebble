use ansi_term::Colour::{Green, Red, Yellow};
use recipe_reader::Recipe;
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

use errors::fail;

pub fn remove(filename: &str) {
	let mut recipe = Recipe::new();
	let mut path = String::new();
	let mut found = false;

	if Recipe::find() != None {
		recipe.read();
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	} else {
		fail("no recipe found in path", 73);
	}

	if !Path::new("pebble.toml").exists() {
		fail("not a valid pebble, missing pebble.toml", 74);
	}
	for t in &mut recipe.targets {
		if t.files.contains(&filename.to_string()) {
			t.files.remove_item(&filename.to_string());
			println!(
				"  {} {} from [{}]",
				Yellow.bold().paint("removed"),
				filename,
				Green.bold().paint(t.name.clone())
			);

			path = filename.to_string();
			found = true;
		} else if t.files.contains(&("src/".to_string() + filename)) {
			t.files.remove_item(&("src/".to_string() + filename));
			println!(
				"  {} {} from [{}]",
				Yellow.bold().paint("removed"),
				filename,
				Green.bold().paint(t.name.clone())
			);

			path = "src/".to_string() + filename;
			found = true;
		}
	}

	if found {
		let cmd = Command::new("git")
			.arg("rm")
			.arg("--cached")
			.arg(&path)
			.output()
			.expect("failed to execute git");
		if !cmd.status.success() {
			println!(
				"  {} git returned a non-zero exit code",
				Red.bold().paint("error")
			);
		}
		recipe.write();
	} else {
		println!("  {} file not found", Red.bold().paint("error"));
	}
}
