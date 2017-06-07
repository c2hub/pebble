use ansi_term::Colour::{Yellow, Red};
use recipe_reader::*;
use std::fs::read_dir;
use std::env::set_current_dir;
use std::path::Path;
use std::ops::Deref;

use errors::*;

pub fn scan()
{
	let mut recipe = Recipe::new();

	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));}
	else
		{fail("no recipe found in path", 82);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 83)}

	for f in match read_dir(Path::new("src"))
		{ Ok(r) => r, _ => fail("failed to open source directory", 84) }
	{
		let file = match f
		{
			Ok(fl) => fl,
			Err(_) => fail("failed to read a directory entry in src", 85)
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
