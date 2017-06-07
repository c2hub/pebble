use commands::build;
use errors::*;

use std::env::{set_current_dir, current_dir};
use ansi_term::Colour::{Yellow, Green};
use std::process::Command;
use recipe_reader::*;

pub fn run(args: Vec<String>)
{
	let orig_cwd = match current_dir()
	{
		Ok(d) => d,
		Err(_) => fail("failed to get current directory path", 75)
	};

	build();

	let mut recipe = Recipe::new();
	recipe.read_errors(true);

	// technically shouldn't occur since build() read the recipe
	// already, but just to be sure... one never knows...
	if !recipe.ok
		{fail("failed to read recipe, exiting", 76);}

	let pebble_name = match recipe.path.parent().unwrap().file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None => fail("could not read name string", 77)
		},
		None => fail("invalid project path", 78)
	};

	if recipe.targets[0].kind != TargetType::Executable
		{fail1("pebble [{}] is a library, did you mean to use 'pebble test'?", Green.bold().paint(pebble_name), 79);}

	let exe_path = match current_dir()
	{
		Ok(cwd) => cwd.into_os_string().into_string().unwrap()
			+ "/output/"
			+ pebble_name
			+ "/"
			+ pebble_name,
		Err(_) =>
			{fail("failed to get current directory", 80)}
	};

	// restore original cwd, so that pebble run can be used from anywhere within the pebble
	if set_current_dir(orig_cwd).is_err()
		{fail("failed to change current directory", 81);};

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
