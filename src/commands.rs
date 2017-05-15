use types::PebbleType;
use util::*;

use std::env::set_current_dir;
use std::fs::create_dir_all;
use std::process::exit;
use std::path::Path;

pub fn new_pebble(path_str: &String, kind: PebbleType)
{
	let proj_path = Path::new(path_str);
	if !proj_path.exists()
	{
		if let Err(_) = create_dir_all(&proj_path)
		{
		    println!("failed to create directory");
		    exit(-1);
		}
		if let Err(_) = set_current_dir(&proj_path)
		{
			println!("failed to change directory");
			exit(-1);
		}
		match kind
		{
			PebbleType::Executable =>
			{

			},
			PebbleType::StaticLib =>
			{

			},
			PebbleType::SharedLib =>
			{

			}
		}
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
