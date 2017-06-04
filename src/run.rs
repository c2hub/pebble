use build::build;

use ansi_term::Colour::{Yellow, Green};
use recipe_reader::*;
use std::env::{set_current_dir, current_dir};
use std::process::Command;
use std::process::exit;

pub fn run(args: Vec<String>)
{
	let orig_cwd = match current_dir()
	{
		Ok(d) => d,
		Err(_) =>
		{
			println!("  error: failed to get current directory path");
			exit(-1);
		}
	};

	build();

	let mut recipe = Recipe::new();
	recipe.read_errors(true);

	// technically shouldn't occur since build() read the recipe
	// already, but just to be sure... one never knows...
	if !recipe.ok
	{
		println!("  error: failed to read recipe, exiting");
		exit(-1);
	}

	let pebble_name = match recipe.path.parent().unwrap().file_stem()
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

	if recipe.targets[0].kind != TargetType::Executable
	{
		println!("  error: pebble [{}] is a library, did you mean to use 'pebble test'?",
			Green.bold().paint(pebble_name)
		);
		exit(-1);
	}

	let exe_path = match current_dir()
	{
		Ok(cwd) => cwd.into_os_string().into_string().unwrap()
			+ "/output/"
			+ pebble_name
			+ "/"
			+ pebble_name,
		Err(_) =>
		{
			println!("  error: failed to get current directory");
			exit(-1);
		}
	};

	// restore original cwd, so that pebble run can be used from anywhere within the pebble
	if set_current_dir(orig_cwd).is_err()
	{
		println!("  error: failed to change current directory");
		exit(-1);
	};

	println!("  {} '{} {}'",
		Yellow.bold().paint("running"),
		&exe_path,
		args.iter().fold(String::new(), |acc, curr| acc + " " + curr.as_ref() )
	);

	Command::new(exe_path)
		.args(args)
		.spawn()
		.expect("  error: failed to launch");
}
