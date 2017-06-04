use types::PebbleType;
use config::Config;
use util::*;
use build::build;

use ansi_term::Colour::{Yellow, Green, Red, Blue};
use recipe_reader::*;
use std::fs::{create_dir_all, create_dir, File, read_dir, copy, remove_file};
use std::env::{set_current_dir, current_dir};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use std::process::exit;
use std::ops::Deref;
use std::io::Write;

pub fn new_pebble(path_str: &str, kind: PebbleType)
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
		Green.bold().paint(path_str)
	);
	if !proj_path.exists()
	{
		if create_dir_all(&proj_path).is_err()
		{
		    println!("  error: failed to create pebble's directory");
		    exit(-1);
		}
		if set_current_dir(&proj_path).is_err()
		{
			println!("  error: failed to change directory");
			exit(-1);
		}
		if create_dir(Path::new("src")).is_err()
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

pub fn init_pebble(path_str: &str, kind: PebbleType)
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
		Green.bold().paint(path_str)
	);
	if proj_path.exists()
	{
		let mut files: Vec<String> = Vec::new();
		if set_current_dir(&proj_path).is_err()
		{
			println!("  error: failed to change directory");
			exit(-1);
		}
		if create_dir(Path::new("src")).is_err()
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
							Some(p) => match p.to_str().unwrap()
							{
								"tests.c2" => continue,
								x => x
							},
							None =>
							{
								println!("  error: failed to read path");
								exit(-1);
							}
						};
						if copy(Path::new(filename), Path::new(&("src/".to_string() + filename))).is_err()
						{
							println!("  error: failed to copy file '{}'", filename);
							exit(-1);
						}
						if remove_file(Path::new(filename)).is_err()
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
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));}
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

pub fn add(filename: &str)
{
	let mut recipe = Recipe::new();

	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));}
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
			t.files.push(filename.to_string());

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

pub fn remove(filename: &str)
{
	let mut recipe = Recipe::new();

	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));}
	else
		{println!("  error: no recipe found in path"); exit(-1);}

	if !Path::new("pebble.toml").exists()
	{
		println!("  error: not a valid pebble, missing pebble.toml");
		exit(-1);
	}
	for mut t in &mut recipe.targets
	{
		if t.files.contains(&filename.to_string())
		|| t.files.contains(&("src/".to_string() + filename))
		{
			t.files.remove_item(&filename.to_string());
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

pub fn test(args: Vec<String>)
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

	if recipe.targets[0].kind == TargetType::Executable
	{
		println!("  error: pebble [{}] is a executable, did you mean to use 'pebble run'?",
			Green.bold().paint(pebble_name)
		);
		exit(-1);
	}

	let exe_path = match current_dir()
	{
		Ok(cwd) => cwd.into_os_string().into_string().unwrap()
			+ "/output/test/test",
		Err(_) =>
		{
			println!("  error: failed to get current directory");
			exit(-1);
		}
	};

	// restore original cwd, so that 'pebble test' can be used from anywhere within the pebble
	// I want to give the choice of the test executable to be either a test suite or an example
	// program using the library
	if set_current_dir(orig_cwd).is_err()
	{
		println!("  error: failed to change current directory");
		exit(-1);
	};

	println!("  {}", Yellow.bold().paint("running tests"));

	Command::new(exe_path)
		.args(args)
		.spawn()
		.expect("  error: failed to launch");
}

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

pub fn uninstall()
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
				Yellow.bold().paint("uninstalling"),
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
			println!("  error: can't uninstall due to missing install instructions");
			exit(-1);
		}
	}
	else
	{
		println!("  error: can't uninstall due to missing build section");
		exit(-1);
	}
}

pub fn help(cmd: &str)
{
	fn general()
	{
		println!("pebble is a tool for managing C2 projects (pebbles) and dependencies\
			  \n\nAvailable commands:\
				\n  pebble new name [type]      create a new pebble\
				\n  pebble init path [type]     make a pebble in an existing directory with\
				\n                              existing files\
				\n  pebble build                build the pebble\
				\n  pebble run                  build & run the pebble\
				\n  pebble test                 build & run tests for the pebble\
				\n  pebble scan                 add all untracked files in 'src/'\
				\n                              remove all non-existant files\
				\n  pebble add file             add a file to the pebble\
				\n  pebble remove file          remove a file from the pebble\
				\n  pebble install              install the pebble\
				\n  pebble uninstall            uninstall the pebble\
				\n  pebble help [command]       show help text, either general\
				\n                              or for a given command\
			  \n\nNote: pebble generates own recipe.txt. Don't touch it or pebble\
			  	\nwill cry. If you want to manipulate files, use appropriate commands\
				 ");
	}
	fn new()
	{
		println!("usage: pebble new name [type]\
			  \n\npebble new creates a new pebble in the directory specified by name.\
				\nthe name will also be the name of the resulting binary and the whole pebble\
			  \n\nIf 'type' isn't specified, pebble makes an executable. The following types are accepted:\
				\n	bin		- makes a new executable\
				\n			- aliases: binary, executable, exe\
				\n	lib		- makes a new static library\
				\n			- aliases: static, libstatic, staticlib, slib\
				\n	dlib	- makes a new dynamic library\
				\n			- aliases: dynamic, libshared, sharedlib, dynamiclib, dlib, shared\
			  \n\nThe newly generated pebble will have three (or four) items in the directory:
				\n  pebble.toml	- pebble's configuration file, see 'pebble help toml' for more information\
				\n  recipe.txt	- recipe file generated by recipe-reader. No touchy-touchy\
				\n  src/		- directory containing source files. main.c2 for bins, lib.c2 for libs\
				\n  tests.c2	- a file containing tests, only for libraries\
			  \n\nNote: While is advised to put your source files into src/ to get the benefits of pebble scan\
				\nand having files organized, you can put them wherever you want and add with pebble 'add'\
				 ");
	}
	fn init()
	{
		println!("usage: pebble init path [type]\
			  \n\npebble initializes a new pebble in the directory specified by path.\
				\nthe lower-most directory in path will be the name of the pebble and resulting binary.\
			 	\npebble will copy all existing C2 source files in the directory to src/ and start tracking them\
			  \n\nIf 'type' isn't specified, pebble makes an executable. The following types are accepted:\
				\n	bin		- makes a new executable\
				\n			- aliases: binary, executable, exe\
				\n	lib		- makes a new static library\
				\n			- aliases: static, libstatic, staticlib, slib\
				\n	dlib	- makes a new dynamic library\
				\n			- aliases: dynamic, libshared, sharedlib, dynamiclib, dlib, shared\
			  \n\nThe newly generated pebble will have three (or four) items in the directory:\
				\n  pebble.toml	- pebble's configuration file, see 'pebble help toml' for more information\
				\n  recipe.txt	- recipe file generated by recipe-reader. No touchy-touchy\
				\n  src/		- directory containing source files. main.c2 for bins, lib.c2 for libs\
				\n  tests.c2	- a file containing tests, only for libraries\
			  \n\nNote: While is advised to put your source files into src/ to get the benefits of pebble scan\
				\nand having files organized, you can put them wherever you want and add with 'pebble add'\
				 ");
	}
	fn build()
	{
		println!("usage: pebble build\
			  \n\nbuilds the pebble producing output in output/pebble_name\
				\ncommands can be executed before and after compilation, if it is\
				\nspecified so in pebble.toml\
				 ");
	}
	fn run()
	{
		println!("usage: pebble run [arguments..]\
			  \n\nBuilds and runs the pebble. Any arguments passed to pebble past 'run'\
				\nare passed to the binary. Working directory in which pebble run is\
				\nran is preserved\
				 ");
	}
	fn test()
	{
		println!("usage: pebble test [arguments..]
				\nbuilds and runs tests of the pebble. Tests are written in tests.c2\
				\npebble test is currently only available. Working directory in which pebble\
				\nis ran is preserved\
				 ");
	}
	fn scan()
	{
		println!("usage: pebble scan\
			  \n\nscans the src directory. All untracked files are are added, all non-existant\
				\nfiles are removed. The files are added to the recipe.txt, which shall not be\
				\ntouched.\
				 ");
	}
	fn add()
	{
		println!("usage pebble add [filename]\
			  \n\nadds a source file to the pebble. pebble tries either filename or src/filename,\
				\nwhichever exists.\
				 ");
	}
	fn remove()
	{
		println!("usage pebble remove [filename]\
			  \n\nremove a source file from the pebble. the file is removed from both main and test\
				\ntargets.\
				 ");
	}
	fn toml()
	{
		println!("pebble uses TOML for it's configuration file. the file has three sections:\
				\n	Package config\
				\n	Library config  (optional)\
				\n	Build config    (optional)\
			  \n\nPackage config:\
				\n	name\
				\n	version\
				\n	source_dir      (optional)\
				\n	license         (optional)\
				\n	dependencies    (optional)\
			  \n\nLibrary config:\
				\n	claim\
				\n	extra\
				\n	version         (optional)\
			  \n\nBuild config:\
				\n	pre             (optional)\
				\n	post            (optional)\
				\n	install         (optional)\
				\n	uninstall       (optional)\
			  \n\ndependencies:\
				\n	name\
				\n	version\
				 ");
	}
	match cmd.as_ref()
	{
		"new" => new(),
		"init" => init(),
		"build" => build(),
		"run" => run(),
		"test" => test(),
		"scan" => scan(),
		"add" => add(),
		"remove" => remove(),
		"toml" => toml(),
		_ => general(),
	}
	exit(0);
}

