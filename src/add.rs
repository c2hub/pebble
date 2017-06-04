use ansi_term::Colour::{Yellow, Green};
use recipe_reader::*;
use std::env::set_current_dir;
use std::path::Path;
use std::process::exit;

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
