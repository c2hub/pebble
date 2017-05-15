extern crate recipe_reader;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate serde_derive;

mod commands;
mod config;
mod types;
mod util;

use commands::*;
use types::*;

use std::process::exit;
use std::env::args;

fn main()
{
	let arguments: Vec<String> = args().collect();
	match arguments[1].as_ref()
	{
		"new" =>
		{
			match arguments.len()
			{
				3 => new_pebble(&arguments[2], PebbleType::Executable),
				4 => match arguments[3].as_ref()
				{
					"lib"|
					"libstatic"|
					"staticlib" =>
						new_pebble(&arguments[2], PebbleType::StaticLib),
					"dynamic"|
					"dynamiclib"|
					"sharedlib"|
					"libshared"|
					"shared" =>
						new_pebble(&arguments[2], PebbleType::SharedLib),
					"executable"|
					"bin"|
					"binary"|
					"exe" =>
						new_pebble(&arguments[2], PebbleType::Executable),
					x =>
					{
						println!("unknown pebble type '{}'", x);
						exit(-1);
					}
				},
				_ =>
				{
					println!("usage: pebble new <name> <type>");
					exit(-1);
				},
			}
		}
		x =>
		{
			println!("unknown operation: '{}'", x);
			exit(-1);
		}
	}
}
