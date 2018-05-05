use errors::{fail, fail1};
use types::PebbleType;
use util::{EXECUTABLE_HELLO_WORLD, EXECUTABLE_RECIPE_STUB, LIB_TEST, PEBBLE_TOML,
           SHAREDLIB_HELLO_WORLD, SHAREDLIB_RECIPE_STUB, STATICLIB_HELLO_WORLD,
           STATICLIB_RECIPE_STUB};

use ansi_term::Colour::{Green, Yellow};
use std::env::set_current_dir;
use std::fs::{create_dir, create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn new_pebble(path_str: &str, kind: PebbleType) {
	let proj_path = Path::new(path_str);
	let name = match proj_path.file_stem() {
		Some(n) => match n.to_str() {
			Some(s) => s,
			None => fail("could not read name string", 44),
		},
		None => fail("invalid project path", 45),
	};
	println!(
		"  {} new pebble [{}] in directory '{}'...",
		Yellow.bold().paint("creating"),
		Green.bold().paint(name),
		Green.bold().paint(path_str)
	);
	if !proj_path.exists() {
		if create_dir_all(&proj_path).is_err() {
			fail("failed to create pebble's directory", 46);
		}
		if set_current_dir(&proj_path).is_err() {
			fail("failed to change directory", 47);
		}
		if create_dir(Path::new("src")).is_err() {
			fail("failed to create source directory", 48);
		}
		match kind {
			PebbleType::Executable => {
				match File::create("recipe.txt") {
					Ok(mut f) => {
						let _ = write!(f, "{}", EXECUTABLE_RECIPE_STUB.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create recipe file", 49),
				};

				match File::create("src/main.c2") {
					Ok(mut f) => {
						let _ = write!(f, "{}", EXECUTABLE_HELLO_WORLD.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create src/main.c2", 50),
				};
			}
			PebbleType::StaticLib => {
				match File::create("recipe.txt") {
					Ok(mut f) => {
						let _ = write!(f, "{}", STATICLIB_RECIPE_STUB.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create recipe file", 51),
				}

				match File::create("src/lib.c2") {
					Ok(mut f) => {
						let _ = write!(f, "{}", STATICLIB_HELLO_WORLD.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create src/lib.c2", 52),
				}

				match File::create("tests.c2") {
					Ok(mut f) => {
						let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create tests.c2", 53),
				}
			}
			PebbleType::SharedLib => {
				match File::create("recipe.txt") {
					Ok(mut f) => {
						let _ = write!(f, "{}", SHAREDLIB_RECIPE_STUB.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create recipe file", 54),
				}

				match File::create("src/lib.c2") {
					Ok(mut f) => {
						let _ = write!(f, "{}", SHAREDLIB_HELLO_WORLD.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create src/lib.c2", 55),
				}

				match File::create("tests.c2") {
					Ok(mut f) => {
						let _ = write!(f, "{}", LIB_TEST.replace("[[name]]", name));
					}
					Err(_) => fail("failed to create tests.c2", 56),
				}
			}
		}

		match File::create("pebble.toml") {
			Ok(mut f) => {
				let _ = write!(f, "{}", PEBBLE_TOML.replace("[[name]]", name));
			}
			Err(_) => fail("failed to create pebble.toml", 57),
		}

		Command::new("git")
			.arg("init")
			.arg(".")
			.output()
			.expect("  error: failed to init git repository");

		println!(
			"  {} creating {} pebble '{}'",
			Yellow.bold().paint("finished"),
			kind.to_string(),
			Green.bold().paint(name)
		);
	} else {
		fail1(
			"'pebble new' is for new pebbles, did you mean to use 'pebble init {}' instead?",
			path_str,
			58,
		);
	}
}
