use ansi_term::Colour::{Blue, Green, Red, Yellow};
use config::Config;
use recipe_reader::Recipe;

use std::env::set_current_dir;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

use errors::fail;

pub fn build() {
	let mut recipe = Recipe::new();

	if Recipe::find() != None {
		recipe.read_errors(true);
		if !recipe.ok {
			fail("failed to read recipe, exiting", 12);
		}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	} else {
		fail("no recipe found in path", 13);
	}

	if !Path::new("pebble.toml").exists() {
		fail("not a valid pebble, missing pebble.toml", 14);
	}

	let cfg = match Config::read() {
		Ok(c) => c,
		Err(_) => fail("failed to parse pebble.toml", 15),
	};

	let name = match recipe.path.parent().unwrap().file_stem() {
		Some(n) => match n.to_str() {
			Some(s) => s,
			None => fail("could not read name string", 16),
		},
		None => fail("invalid project path", 17),
	};

	if let Some(ref bcfg) = cfg.build {
		if let Some(ref pre) = bcfg.pre {
			for cmd_str in pre {
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
		}
	}

	println!(
		"  {} [{}]",
		Yellow.bold().paint("compiling"),
		Green.bold().paint(name)
	);

	let output = Command::new("c2c")
		.arg(name)
		.output()
		.expect("  error: failed to execute c2c");

	if let Some(ref bcfg) = cfg.build {
		if let Some(ref post) = bcfg.post {
			for cmd_str in post {
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
		}
	}

	if !output.status.success() {
		println!("  {} during compilation:\n", Red.bold().paint("error"));
		unsafe {
			print!(
				"{}",
				// on one hand, I feel smart for knowing how to repaint the string
				// in one function call, on the other, I feel retarded for having
				// to do this, hopefully I will figure out a better solution later
				String::from_utf8_unchecked(output.stderr.to_vec())
					.lines()
					.fold(String::new(), |acc, current| {
						if current.contains("error") || current.contains(".c2:") {
							acc + &Red.bold().paint(current).to_string() + "\n"
						} else if current.contains('^') {
							acc + &Green.bold().paint(current).to_string() + "\n"
						} else {
							acc + current + "\n"
						}
					})
			);
		}
		exit(-1);
	} else {
		println!("{}", Yellow.bold().paint("  build succeeded"));
	}
}
