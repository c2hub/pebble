use types::PebbleType;
use std::fs::create_dir_all;
use std::process::exit;
use std::path::Path;

pub fn new_pebble(path_str: &String, kind: PebbleType)
{
	let proj_path = Path::new(path_str);
	if !proj_path.exists()
	{
		match create_dir_all(&proj_path)
		{
			Err(_) =>
			{
				println!("failed to create directory");
				exit(-1);
			},
			_ => {},
		}
	}
	else
	{
		println!("directory '{0}' already exists, did you mean to use 'pebble init {0}' instead", path_str); 
		exit(-1);
	}
}
