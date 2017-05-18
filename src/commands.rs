use types::PebbleType;
use util::*;

use ansi_term::Colour::{Yellow, Green};

use std::process::Command;
use std::env::{set_current_dir, current_dir};
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
	println!("  {} new pebble [{}] in directory '{}'...", 
		Yellow.bold().paint("creating"),
		Green.bold().paint(name),
		Green.bold().paint(path_str.clone())
	);
	if !proj_path.exists()
	{
		if let Err(_) = create_dir_all(&proj_path)
		{
		    println!("  error: failed to create pebble's directory");
		    exit(-1);
		}
		if let Err(_) = set_current_dir(&proj_path)
		{
			println!("  error: failed to change directory");
			exit(-1);
		}
		if let Err(_) = create_dir(Path::new("src"))
		{
			println!("  error: failed to create source directory");
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
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				};
				
				match File::create("src/main.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", EXECUTABLE_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/main.c2");
						exit(-1);
					}
				};
			},
			PebbleType::StaticLib =>
			{
				match File::create("recipe.txt")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", STATICLIB_RECIPE_STUB.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				}

				match File::create("src/lib.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", STATICLIB_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/lib.c2");
						exit(-1);
					}
				}

				match File::create("tests.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create tests.c2");
						exit(-1);
					}
				}
			},
			PebbleType::SharedLib =>
			{
				match File::create("recipe.txt")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", SHAREDLIB_RECIPE_STUB.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				}

				match File::create("src/lib.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", SHAREDLIB_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/lib.c2");
						exit(-1);
					}
				}

				match File::create("tests.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create tests.c2");
						exit(-1);
					}
				}
			}
		}

		match File::create("pebble.toml")
		{
			Ok(mut f) =>
				{let _ = write!(f, "{}", PEBBLE_TOML.replace("[[name]]", name));},
			Err(_) =>
			{
				println!("  error: failed to create pebble.toml");
				exit(-1);
			}
		}
		
		Command::new("git")
			.arg("init")
			.arg(".")
			.output()
			.expect("  error: failed to init git repository");

		println!("  {} creating {} pebble '{}'", 
			Yellow.bold().paint("finished"),
			kind.to_string(),
			Green.bold().paint(name)
		);
	}
	else
	{
		println!(
			"  error: directory '{0}' already exists, did you mean to use 'pebble init {0}' instead?",
			path_str
		); 
		exit(-1);
	}
}

//TODO
pub fn init_pebble(path_str: &String, kind: PebbleType)
{
	let cwd = match current_dir()
	{
		Ok(s) => s,
		Err(_) =>
		{
			println!("  error: could not access current directory");
			exit(-1);
		}
	};
	let proj_path = match path_str.as_ref()
	{
		"." => Path::new(&cwd),
		".." => match cwd.parent()
		{
			Some(s) => s,
			None =>
			{
				println!("  error: could not access current directory");
				exit(-1);
			} 
		},
		_ => Path::new(path_str),
	};
	let name = match proj_path.file_stem()
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
	println!("  {} new pebble [{}] in directory '{}'...", 
		Yellow.bold().paint("initializing"),
		Green.bold().paint(name),
		Green.bold().paint(path_str.clone())
	);
	if proj_path.exists()
	{
		if let Err(_) = set_current_dir(&proj_path)
		{
			println!("  error: failed to change directory");
			exit(-1);
		}
		if let Err(_) = create_dir(Path::new("src"))
		{
			println!("  error: failed to create source directory");
			exit(-1);
		}
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
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				};
				
				match File::create("src/main.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", EXECUTABLE_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/main.c2");
						exit(-1);
					}
				};
			},
			PebbleType::StaticLib =>
			{
				match File::create("recipe.txt")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", STATICLIB_RECIPE_STUB.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				}

				match File::create("src/lib.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", STATICLIB_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/lib.c2");
						exit(-1);
					}
				}

				match File::create("tests.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create tests.c2");
						exit(-1);
					}
				}
			},
			PebbleType::SharedLib =>
			{
				match File::create("recipe.txt")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", SHAREDLIB_RECIPE_STUB.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create recipe file");
						exit(-1);
					}
				}

				match File::create("src/lib.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", SHAREDLIB_HELLO_WORLD.replace("[[name]]", name));},
					Err(_) =>
					{
						println!("  error: failed to create src/lib.c2");
						exit(-1);
					}
				}

				match File::create("tests.c2")
				{
					Ok(mut f) =>
						{let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));}
					Err(_) =>
					{
						println!("  error: failed to create tests.c2");
						exit(-1);
					}
				}
			}
		}

		match File::create("pebble.toml")
		{
			Ok(mut f) =>
				{let _ = write!(f, "{}", PEBBLE_TOML.replace("[[name]]", name));},
			Err(_) =>
			{
				println!("  error: failed to create pebble.toml");
				exit(-1);
			}
		}
		
		Command::new("git")
			.arg("init")
			.arg(".")
			.output()
			.expect("  error: failed to init git repository");

		println!("  {} creating {} pebble '{}'", 
			Yellow.bold().paint("finished"),
			kind.to_string(),
			Green.bold().paint(name)
		);
	}
	else
	{
		println!(
			"  error: directory '{0}' already exists, did you mean to use 'pebble init {0}' instead?",
			path_str
		); 
		exit(-1);
	}
}

