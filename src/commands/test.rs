use commands::build;
use errors::*;

use ansi_term::Colour::{Yellow, Green};
use recipe_reader::*;
use std::env::{set_current_dir, current_dir};
use std::process::Command;

pub fn test(args: Vec<String>)
{
	let orig_cwd = match current_dir()
	{
		Ok(d) => d,
		Err(_) => fail("failed to get current directory path", 86)
	};

	build();

	let mut recipe = Recipe::new();
	recipe.read_errors(true);

	// technically shouldn't occur since build() read the recipe
	// already, but just to be sure... one never knows...
	if !recipe.ok
		{fail("failed to read recipe, exiting", 87)}

	let pebble_name = match recipe.path.parent().unwrap().file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None => fail("could not read name string", 88)
		},
		None => fail("invalid project path", 89)
	};

	if recipe.targets[0].kind == TargetType::Executable
		{fail1("pebble [{}] is a executable, did you mean to use 'pebble run'?", Green.bold().paint(pebble_name), 90);}

	let exe_path = match current_dir()
	{
		Ok(cwd) => cwd.into_os_string().into_string().unwrap()
			+ "/output/test/test",
		Err(_) => fail("failed to get current directory", 91)
	};

	// restore original cwd, so that 'pebble test' can be used from anywhere within the pebble
	// I want to give the choice of the test executable to be either a test suite or an example
	// program using the library
	if set_current_dir(orig_cwd).is_err()
		{fail("failed to change current", 92);}

	println!("  {}", Yellow.bold().paint("running tests"));

	Command::new(exe_path)
		.args(args)
		.spawn()
		.expect("  error: failed to launch");
}
