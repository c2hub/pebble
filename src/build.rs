use config::Config;
use ansi_term::Colour::{Yellow, Green, Red, Blue};
use recipe_reader::*;

use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::process::exit;

pub fn build()
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
		if let Some(ref pre) = bcfg.pre
		{
			for cmd_str in pre
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
	}

	println!("  {} [{}]",
		Yellow.bold().paint("compiling"),
		Green.bold().paint(name)
	);

	let output = Command::new("c2c")
		.arg(name)
		.output()
		.expect("  error: failed to execute c2c");

	if let Some(ref bcfg) = cfg.build
	{
		if let Some(ref post) = bcfg.post
		{
			for cmd_str in post
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
	}

	if !output.status.success()
	{
		println!("  {} during compilation:\n", Red.bold().paint("error"));
		unsafe { print!("{}",
			// on one hand, I feel smart for knowing how to repaint the string
			// in one function call, on the other, I feel retarded for having
			// to do this, hopefully I will figure out a better solution later
			String::from_utf8_unchecked(output.stderr.to_vec()).lines().fold
			(	String::new(),
				|acc, current|
				{
					if current.contains("error")
					|| current.contains(".c2:")
					{
						acc + &Red.bold().paint(current).to_string() + "\n"
					}
					else if current.contains('^')
					{
						acc + &Green.bold().paint(current).to_string() + "\n"
					}
					else
					{
					   	acc + current + "\n"
					}
				}
			)
		); }
		exit(-1);
	}
	else
		{println!("{}", Yellow.bold().paint("  build succeeded"));}
}
