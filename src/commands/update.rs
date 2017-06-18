use ansi_term::Colour::Yellow;
/*use hyper::client::Client;*/
use recipe_reader::Recipe;

/*use std::io::Read;*/
use std::path::Path;
use std::env::set_current_dir;

use errors::fail;

pub fn update()
{
	let mut _recipe = Recipe::new();

	if Recipe::find() != None
		{_recipe.read(); let _ = set_current_dir(Path::new(&_recipe.path.parent().unwrap()));}
	else
		{fail("no recipe found in path",9);}

	if !Path::new("pebble.toml").exists()
		{fail("not a valid pebble, missing pebble.toml", 10);}

	println!("  {} http://magnusi.tech/static/pebbles/data/index for pebbles",
		Yellow.bold().paint("scanning")
	);

	/*let mut index = match Client::new().get("http://magnusi.tech/static/pebbles/data/index").send()
	{
		Ok(res) => res,
		Err(_) => fail("failed to acquire pebble index, are you connected to the internet?", 101)
	};

	println!("{}", { let mut s = String::new(); let _ = index.read_to_string(&mut s); s});*/
}
