use types::PebbleType;
use util::*;

use std::process::Command;
use std::env::set_current_dir;
use std::fs::{create_dir_all, create_dir, File};
use std::io::Write;
use std::process::exit;
use std::path::Path;

pub fn new_pebble(path_str: &String, kind: PebbleType)
{
	let proj_path = Path::new(path_str);
	let name = match proj_path.file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None =>
			{
				println!("could not read name string");
				exit(-1);
			}
		},
		None =>
		{
			println!("invalid project path");
			exit(-1);
		}
	};
	if !proj_path.exists()
	{
		if let Err(_) = create_dir_all(&proj_path)
		{
		    println!("failed to create pebble's directory");
		    exit(-1);
		}
		if let Err(_) = set_current_dir(&proj_path)
		{
			println!("failed to change directory");
			exit(-1);
		}
		if let Err(_) = create_dir(Path::new("src"))
		{
			println!("failed to create source directory");
			exit(-1);
		}
		//TODO
		match kind
		{
			PebbleType::Executable =>
			{
				match File::create("recipe.txt")
				{
					Ok(mut f) => 
						{let _ = write!(f, "{}", EXECUTABLE_RECIPE_STUB.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("failed to create recipe file");
						exit(-1);
					}
				};
				
				match File::create("src/main.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", C2_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("failed to create src/main.c2");
						exit(-1);
					}
				};
			},
			PebbleType::StaticLib =>
			{

			},
			PebbleType::SharedLib =>
			{

			}
		}

		match File::create("pebble.toml")
		{
			Ok(mut f) =>
				{let _ = write!(f, "{}", PEBBLE_TOML.replace("[[name]]", name));},
			Err(_) =>
			{
				println!("failed to create pebble.toml");
				exit(-1);
			}
		}
		
		Command::new("git")
			.arg("init")
			.arg(".")
			.spawn()
			.expect("failed to init git repository");
	}
	else
	{
		println!(
			"directory '{0}' already exists, did you mean to use 'pebble init {0}' instead?",
			path_str
		); 
		exit(-1);
	}
}