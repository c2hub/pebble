use util::{INIT_TEST, PEBBLE_TOML};
use errors::{fail, fail1};
use types::PebbleType;


use recipe_reader::{Recipe, TargetOptions, TargetType, Target};
use std::fs::{create_dir, File, read_dir, copy, remove_file};
use std::env::{set_current_dir, current_dir};
use ansi_term::Colour::{Yellow, Green};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Write;

pub fn init_pebble(path_str: &str, kind: PebbleType)
{
	let cwd = match current_dir()
	{
		Ok(s) => s,
		Err(_) => fail("could not access current directory", 19)
	};
	let proj_path = match path_str.as_ref()
	{
		"." => Path::new(&cwd),
		".." => match cwd.parent()
		{
			Some(s) => s,
			None => fail("could not access parent directory", 20)
		},
		_ => Path::new(path_str),
	};
	let name = match proj_path.file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None => fail("could not read name string", 21)
		},
		None => fail("invalid project path", 22)
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
			{fail("failed to change directory", 23);}
		if create_dir(Path::new("src")).is_err()
			{fail("failed to create source directory", 24)}
		for f in match read_dir(Path::new("."))
			{ Ok(r) => r, _ => fail("failed to open current directory", 25)}
		{
			match f
			{
				Ok(f) =>
				{
					if match f.metadata()
					{
						Ok(m) => m.is_file(),
						Err(_) => fail("failed to read file metadata", 26)
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
							None => fail("failed to read path", 27)
						};
						if copy(Path::new(filename), Path::new(&("src/".to_string() + filename))).is_err()
							{fail1("failed to copy file '{}'", filename, 28);}
						if remove_file(Path::new(filename)).is_err()
							{fail1("failed to move file '{}'", filename, 29);}
						files.push("src/".to_string() + filename);
					}
				},
				Err(_) => fail("failed to open read file in directory", 30)
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
						Err(_) => fail("failed to create tests.c2", 31)
					}
				}
			}
		}

		match File::create("pebble.toml")
		{
			Ok(mut f) =>
				{let _ = write!(f, "{}", PEBBLE_TOML.replace("[[name]]", name));},
			Err(_) => fail("failed to create pebble.toml", 32)
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
		{fail1("'pebble init' is for existing directories, did you mean to use 'pebble new {}' instead", path_str, 33);}
}
