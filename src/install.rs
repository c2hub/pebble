use config::Config;

use ansi_term::Colour::{Yellow, Green, Red, Blue};
use recipe_reader::*;
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::process::exit;

pub fn install()
{
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

	let name = match recipe.path.parent().unwrap().file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None =>
			{
				println!("  error: could not read name string");
				exit(-1);
			}
		},
		None =>
		{
			println!("  error: invalid project path");
			exit(-1);
		}
	};

	if let Some(ref bcfg) = cfg.build
	{
		if let Some(ref install) = bcfg.install
		{
			println!("  {} [{}]",
				Yellow.bold().paint("installing"),
				Green.bold().paint(name)
			);

			for cmd_str in install
			{
				println!("  {} '{}'",
					Yellow.bold().paint("executing"),
					&cmd_str
				);
				let name = cmd_str.split_whitespace().collect::<Vec<&str>>()[0];
				let args: Vec<&str> = cmd_str.split_whitespace().skip(1).collect();
				let cmd = Command::new(name)
					.args(args)
					.stdout(Stdio::inherit())
					.stderr(Stdio::inherit())
					.output()
					.expect("  error: failed to run command");
				if !cmd.status.success()
				{
					println!("  {}: commmand '{}' returned non-zero exit code",
						Red.bold().paint("warning"),
						Blue.bold().paint(cmd_str.clone())
					);
				}
			}
		}
		else
		{
			println!("  error: can't install due to missing install instructions");
			exit(-1);
		}
	}
	else
	{
		println!("  error: can't install due to missing build section");
		exit(-1);
	}
}
