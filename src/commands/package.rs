use commands::build;
use errors::{fail, fail1};

use ansi_term::Colour::{Yellow, Green};
use walkdir::WalkDirIterator;
use recipe_reader::Recipe;
use walkdir;
use zip;

use std::env::{set_current_dir};
use std::fs::{File, metadata};
use std::path::{Path};
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
			{fail("failed to read recipe, exiting", 59);}
		let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));
	}
	else
		{fail("no recipe found in path", 60);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 61);}

	let name = match recipe.path.parent().unwrap().file_stem()
	{
		Some(n) => match n.to_str()
		{
			Some(s) => s,
			None => fail("could not read name string", 62)
		},
		None => fail("invalid project path", 63)
	};

	println!("  {} pebble [{}]",
		Yellow.bold().paint("packaging"),
		Green.bold().paint(name)
	);

	let mut zip_f = match File::create("package.zip")
	{
		Ok(f) => f,
		Err(_) => fail("failed to create package file", 64)
	};

	let mut zip = zip::ZipWriter::new(&mut zip_f);

	for entry in walkdir::WalkDir::new(Path::new("."))
		.into_iter()
		.filter_entry(
			|e| !(e.file_name().to_str().unwrap().contains(".git")
				|| e.path().to_str().unwrap().contains("output")
				|| e.path().to_str().unwrap().contains("package.zip")
				|| e.path().to_str().unwrap().contains("libs")
				|| e.file_name().to_str().unwrap().ends_with('/'))
			)
	{
		let entry = match entry
		{
			Ok(ent) => ent,
			Err(e) => fail1("failed to read directory entry, {}", e, 65)
		};

		if metadata(entry.path()).unwrap().is_dir()
			{continue;}

		if zip.start_file(entry.path().to_str().unwrap(), zip::write::FileOptions::default()).is_err()
			{fail("failed to start a zip", 66);}

		let mut f = match File::open(entry.path())
		{
			Ok(f) => f,
			Err(_) => fail("failed to open directory entry for reading", 67)
		};

		let mut content = String::new();

		if f.read_to_string(&mut content).is_ok()
		&& zip.write(&{ let v: Vec<u8> = content.bytes().collect(); v }).is_err()
			{fail("failed to write file to zip", 68);}
	}

	if zip.finish().is_err()
		{fail("failed to finish writing zip",69);}

	let mut zip_f = match File::open("package.zip")
	{
		Ok(f) => f,
		Err(_) => fail("failed to open zip file for reading", 70)
	};

	let mut bytes = Vec::new();
	if zip_f.read_to_end(&mut bytes).is_err()
		{fail("failed to read zip file", 71);}

	bytes
}
