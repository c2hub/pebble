use types::PebbleType;
use util::*;

use ansi_term::Colour::{Yellow, Green, Red};
use recipe_reader::*;

use std::process::Command;
use std::env::{set_current_dir, current_dir};
use std::fs::{create_dir_all, create_dir, File, read_dir, copy, remove_file};
use std::io::Write;
use std::process::exit;
use std::path::{Path, PathBuf};
use std::ops::Deref;

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
		let mut files: Vec<String> = Vec::new();
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
		for f in match read_dir(Path::new("."))
			{ Ok(r) => r, _ => {println!(" error: failed to open current directory"); exit(-1);} }
		{
			match f
			{
				Ok(f) =>
				{
					if match f.metadata()
					{
						Ok(m) => m.is_file(),
						Err(_) =>
						{
							println!("  error: failed to read file metadata");
							exit(-1);
						}
					}
					&& match Path::new(&f.path()).extension() {Some(ex) => ex == "c2", None => {false}} 
					{
						let path = f.path();
						let filename = match path.file_name()
						{
							Some(p) => p.to_str().unwrap(),
							None =>
							{
								println!("  error: failed to read path");
								exit(-1);
							}
						};
						if let Err(_) = copy(Path::new(filename), Path::new(&("src/".to_string() + filename)))
						{
							println!("  error: failed to copy file '{}'", filename);
							exit(-1);
						}
						if let Err(_) = remove_file(Path::new(filename))
						{
							println!("  error: failed to move file '{}'", filename);
							exit(-1);
						}
						files.push("src/".to_string() + filename);
					}
				},
				Err(_) =>
				{
					println!("  error: failed to open read file in directory");
					exit(-1);
				} 
			}
		}

		match kind
		{
			PebbleType::Executable =>
			{
				let mut recipe = Recipe::new();
				recipe.targets.push(
					Target
					{
						name: name.to_string(),
						kind: TargetType::Executable,
						files: files,
						options: TargetOptions
						{
							deps: false,
							refs: false,
							nolibc: false,
							generate_c: true,
							generate_ir: false,
							lib_use: Vec::new(),
							export: Vec::new(),
							config: Vec::new(),
							warnings: Vec::new(),
						}
					}
				);
				recipe.write_path(&PathBuf::from("./recipe.txt"));
			},
			t@ PebbleType::StaticLib | t@ PebbleType::SharedLib =>
			{
				let mut recipe = Recipe::new();
				recipe.targets.push(
					Target
					{
						name: name.to_string(),
						kind: match t
						{
							PebbleType::StaticLib => TargetType::StaticLib,
							PebbleType::SharedLib => TargetType::SharedLib,
							_ => unreachable!(), //already certain it's a library, heh
						},
						files: files.clone(),
						options: TargetOptions
						{
							deps: false,
							refs: false,
							nolibc: false,
							generate_c: true,
							generate_ir: false,
							lib_use: Vec::new(),
							export: vec![name.to_string()],
							config: Vec::new(),
							warnings: Vec::new(),
						}
					}
				);
				files.push("tests.c2".to_string());
				recipe.targets.push(
					Target
					{
						name: name.to_string(),
						kind: TargetType::Executable,
						files: files,
						options: TargetOptions
						{
							deps: false,
							refs: false,
							nolibc: false,
							generate_c: true,
							generate_ir: false,
							lib_use: Vec::new(),
							export: Vec::new(),
							config: Vec::new(),
							warnings: Vec::new(),
						}
					}
				);
				recipe.write_path(&PathBuf::from("./recipe.txt"));

				if !Path::new("tests.c2").exists()
				{
					match File::create("tests.c2")
					{
						Ok(mut f) =>
							{let _ = write!(f, "{}", INIT_TEST);}
						Err(_) =>
						{
							println!("  error: failed to create tests.c2");
							exit(-1);
						}
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

		println!("  {} initializing {} pebble '{}'", 
			Yellow.bold().paint("finished"),
			kind.to_string(),
			Green.bold().paint(name)
		);
	}
	else
	{
		println!(
			"  error: 'pebble init' is for existing directories, did you mean to use 'pebble new {0}' instead?",
			path_str
		); 
		exit(-1);
	}
}

pub fn scan()
{
	let mut recipe = Recipe::new();
	
	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path));}
	else
		{println!("  error: no recipe found in path"); exit(-1);}
	
	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}

	for f in match read_dir(Path::new("src"))
		{ Ok(r) => r, _ => {println!(" error: failed to open source directory"); exit(-1);} }
	{
		let file = match f
		{
			Ok(fl) => fl,
			Err(_) =>
			{
				println!("  error: failed to read a directory entry in src");
				exit(-1);
			}
		};

		if !recipe.targets[0].files.contains(&file.path().to_string_lossy().deref().to_string())
		{
			for mut t in &mut recipe.targets
				{t.files.insert(0, file.path().to_string_lossy().deref().to_string());}
			println!("  {} {}",
				Yellow.bold().paint("adding"),
				file.path().to_string_lossy(),
			);
		}
	}

	for mut t in &mut recipe.targets
	{
		for file in t.files.clone()
		{
			if !Path::new(&file).exists()
			{
				t.files.remove_item(&file);
				println!("  {} {}",
					Red.bold().paint("removing"),
					file,
				);
			}
		}
	}

	recipe.write();
}

pub fn add(filename: &String)
{
	let mut recipe = Recipe::new();
	
	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path));}
	else
		{println!("  error: no recipe found in path"); exit(-1);}
	
	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}

	if Path::new(filename).exists()
	{
		for mut t in &mut recipe.targets
		{
			t.files.push(filename.clone());

			println!("  {} {} to [{}]",
				Yellow.bold().paint("added"),
				filename,
				Green.bold().paint(t.name.clone())
			);
		}
	}
	else if !Path::new(filename).exists()
	&& Path::new(&("src/".to_string() + filename)).exists()
	{
		for mut t in &mut recipe.targets
		{
			t.files.push("src/".to_string() + filename);

			println!("  {} src/{} to [{}]",
				Yellow.bold().paint("added"),
				filename,
				Green.bold().paint(t.name.clone())
			);
		}
	}
	else
	{
		println!("  error: '{}' not found", filename);
		exit(-1);
	}

	recipe.write();
}

pub fn remove(filename: &String)
{
	let mut recipe = Recipe::new();
	
	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path));}
	else
		{println!("  error: no recipe found in path"); exit(-1);}
	
	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}
	for mut t in &mut recipe.targets
	{
		if t.files.contains(filename)
		|| t.files.contains(&("src/".to_string() + filename))
		{
			t.files.remove_item(&filename);
			t.files.remove_item(&("src/".to_string() + filename));
			println!("  {} {} from [{}]",
				Yellow.bold().paint("removed"),
				filename,
				Green.bold().paint(t.name.clone())
			);
		}
	}
	recipe.write();
}

pub fn build()
{
	let mut recipe = Recipe::new();
	
	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path));}
	else
		{println!("  error: no recipe found in path"); exit(-1);}
	
	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}

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

	println!("  {} [{}]",
		Yellow.bold().paint("compiling"),
		Green.bold().paint(name)
	);

	let output = Command::new("c2c")
		.arg(name)
		.output()
		.expect("  error: failed to execute c2c");
	
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
						return acc + &Red.bold().paint(current).to_string() + "\n";
					}
					else if current.contains("^")
					{
						return acc + &Green.bold().paint(current).to_string() + "\n";
					}
					else
					{
					   	return acc + current + "\n";
					}
				}
			)
		); }
		exit(-1);
	}
	else
		{println!(" {}", Yellow.bold().paint("  build succeeded"));}
}
