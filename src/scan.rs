use ansi_term::Colour::{Yellow, Red};
use recipe_reader::*;
use std::fs::read_dir;
use std::env::set_current_dir;
use std::path::Path;
use std::process::exit;
use std::ops::Deref;

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
