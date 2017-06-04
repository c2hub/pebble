use build::build;

use ansi_term::Colour::{Yellow, Green};
use walkdir::WalkDirIterator;
use recipe_reader::*;
use walkdir;
use zip;

use std::fs::{File, metadata};
use std::env::{set_current_dir};
use std::path::{Path};
use std::process::exit;
use std::io::Write;
use std::io::Read;

pub fn package() -> Vec<u8>
{
	build();

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

	println!("  {} pebble [{}]",
		Yellow.bold().paint("packaging"),
		Green.bold().paint(name)
	);

	let mut zip_f = match File::create("package.zip")
	{
		Ok(f) => f,
		Err(_) =>
		{
			println!("  error: failed to create package file");
			exit(-1);
		}
	};

	let mut zip = zip::ZipWriter::new(&mut zip_f);

	for entry in walkdir::WalkDir::new(Path::new("."))
		.into_iter()
		.filter_entry(
			|e| !(e.file_name().to_str().unwrap().contains(".git")
				|| e.path().to_str().unwrap().contains("output")
				|| e.path().to_str().unwrap().contains("package.zip")
				|| e.file_name().to_str().unwrap().ends_with("/"))
			)
	{
		let entry = match entry
		{
			Ok(ent) => ent,
			Err(e) =>
			{
				println!("  error: failed to read directory entry, {}", e);
				exit(-1);
			}
		};

		if metadata(entry.path()).unwrap().is_dir()
			{continue;}

		if let Err(_) = zip.start_file(entry.path().to_str().unwrap(), zip::write::FileOptions::default())
		{
			println!("  error: failed to start a zip");
			exit(-1);
		}

		let mut f = match File::open(entry.path())
		{
			Ok(f) => f,
			Err(_) =>
			{
				println!("  error: failed to open directory entry for reading");
				exit(-1);
			}
		};

		let mut content = String::new();

		if let Ok(_) = f.read_to_string(&mut content)
		{
			if let Err(_) = zip.write(&{ let v: Vec<u8> = content.bytes().collect(); v })
			{
				println!("  error: failed to write file to zip");
				exit(-1);
			}
		}
	}

	if let Err(_) = zip.finish()
	{
		println!("  error: failed to finish writing zip");
		exit(-1);
	}

	let mut zip_f = match File::open("package.zip")
	{
		Ok(f) => f,
		Err(_) =>
		{
			println!("  error: failed to open zip file for reading");
			exit(-1);
		}
	};

	let mut bytes = Vec::new();
	if let Err(_) = zip_f.read_to_end(&mut bytes)
	{
		println!("  error: failed to read zip file");
		exit(-1);
	}

	bytes
}
