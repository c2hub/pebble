use ansi_term::Colour::{Yellow, Green};
use std::env::set_current_dir;
use recipe_reader::Recipe;
use std::path::Path;

use errors::fail;

pub fn remove(filename: &str)
{
	let mut recipe = Recipe::new();

	if Recipe::find() != None
		{recipe.read(); let _ = set_current_dir(Path::new(&recipe.path.parent().unwrap()));}
	else
		{fail("no recipe found in path", 73);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 74);}
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
