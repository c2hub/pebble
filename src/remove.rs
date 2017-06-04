use ansi_term::Colour::{Yellow, Green};
use recipe_reader::*;
use std::env::set_current_dir;
use std::path::Path;
use std::process::exit;

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
