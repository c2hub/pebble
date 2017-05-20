#![feature(vec_remove_item)]

extern crate recipe_reader;
extern crate ansi_term;
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
		"init" => match arguments.len()
		{
			3 => init_pebble(&arguments[2], PebbleType::Executable),
			4 => match arguments[3].as_ref()
			{
				"lib"|
				"libstatic"|
				"staticlib" =>
					init_pebble(&arguments[2], PebbleType::StaticLib),
				"dynamic"|
				"dynamiclib"|
				"sharedlib"|
				"libshared"|
				"shared" =>
					init_pebble(&arguments[2], PebbleType::SharedLib),
				"executable"|
				"bin"|
				"binary"|
				"exe" =>
					init_pebble(&arguments[2], PebbleType::Executable),
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
		},
		"scan" => match arguments.len()
		{
			2 => scan(),
			_ =>
			{
				println!("usage: pebble scan #what else?");
				exit(-1);
			}
		},
		"add" => match arguments.len()
		{
			3 => add(&arguments[2]),
			_ =>
			{
				println!("usage: pebble add 'filename'");
				exit(-1);
			}
		},
		"remove" | "del" => match arguments.len()
		{
			3 => remove(&arguments[2]),
			_ =>
			{
				println!("usage: pebble remove 'filename'");
				exit(-1);
			}
		},
		"build" => match arguments.len()
		{
			2 => build(),
			_ =>
			{
				println!("usage: pebble build");
				exit(-1);
			}
		},
		"run" => match arguments.len()
		{
			_ => run(arguments.clone().into_iter().skip(2).collect()),
		},
		"test" => match arguments.len()
		{
			_ => test(arguments.clone().into_iter().skip(2).collect()),
		},
		x =>
		{
			println!("unknown operation: '{}'", x);
			exit(-1);
		}
	}
}
