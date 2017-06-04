use std::process::exit;

pub fn help(cmd: &str)
{
	fn general()
	{
		println!("pebble is a tool for managing C2 projects (pebbles) and dependencies\
			  \n\nAvailable commands:\
				\n  pebble new name [type]      create a new pebble\
				\n  pebble init path [type]     make a pebble in an existing directory with\
				\n                              existing files\
				\n  pebble build                build the pebble\
				\n  pebble run                  build & run the pebble\
				\n  pebble test                 build & run tests for the pebble\
				\n  pebble scan                 add all untracked files in 'src/'\
				\n                              remove all non-existant files\
				\n  pebble add file             add a file to the pebble\
				\n  pebble remove file          remove a file from the pebble\
				\n  pebble install              install the pebble\
				\n  pebble uninstall            uninstall the pebble\
				\n  pebble help [command]       show help text, either general\
				\n                              or for a given command\
			  \n\nNote: pebble generates own recipe.txt. Don't touch it or pebble\
			  	\nwill cry. If you want to manipulate files, use appropriate commands\
				 ");
	}
	fn new()
	{
		println!("usage: pebble new name [type]\
			  \n\npebble new creates a new pebble in the directory specified by name.\
				\nthe name will also be the name of the resulting binary and the whole pebble\
			  \n\nIf 'type' isn't specified, pebble makes an executable. The following types are accepted:\
				\n	bin		- makes a new executable\
				\n			- aliases: binary, executable, exe\
				\n	lib		- makes a new static library\
				\n			- aliases: static, libstatic, staticlib, slib\
				\n	dlib	- makes a new dynamic library\
				\n			- aliases: dynamic, libshared, sharedlib, dynamiclib, dlib, shared\
			  \n\nThe newly generated pebble will have three (or four) items in the directory:
				\n  pebble.toml	- pebble's configuration file, see 'pebble help toml' for more information\
				\n  recipe.txt	- recipe file generated by recipe-reader. No touchy-touchy\
				\n  src/		- directory containing source files. main.c2 for bins, lib.c2 for libs\
				\n  tests.c2	- a file containing tests, only for libraries\
			  \n\nNote: While is advised to put your source files into src/ to get the benefits of pebble scan\
				\nand having files organized, you can put them wherever you want and add with pebble 'add'\
				 ");
	}
	fn init()
	{
		println!("usage: pebble init path [type]\
			  \n\npebble initializes a new pebble in the directory specified by path.\
				\nthe lower-most directory in path will be the name of the pebble and resulting binary.\
			 	\npebble will copy all existing C2 source files in the directory to src/ and start tracking them\
			  \n\nIf 'type' isn't specified, pebble makes an executable. The following types are accepted:\
				\n	bin		- makes a new executable\
				\n			- aliases: binary, executable, exe\
				\n	lib		- makes a new static library\
				\n			- aliases: static, libstatic, staticlib, slib\
				\n	dlib	- makes a new dynamic library\
				\n			- aliases: dynamic, libshared, sharedlib, dynamiclib, dlib, shared\
			  \n\nThe newly generated pebble will have three (or four) items in the directory:\
				\n  pebble.toml	- pebble's configuration file, see 'pebble help toml' for more information\
				\n  recipe.txt	- recipe file generated by recipe-reader. No touchy-touchy\
				\n  src/		- directory containing source files. main.c2 for bins, lib.c2 for libs\
				\n  tests.c2	- a file containing tests, only for libraries\
			  \n\nNote: While is advised to put your source files into src/ to get the benefits of pebble scan\
				\nand having files organized, you can put them wherever you want and add with 'pebble add'\
				 ");
	}
	fn build()
	{
		println!("usage: pebble build\
			  \n\nbuilds the pebble producing output in output/pebble_name\
				\ncommands can be executed before and after compilation, if it is\
				\nspecified so in pebble.toml\
				 ");
	}
	fn run()
	{
		println!("usage: pebble run [arguments..]\
			  \n\nBuilds and runs the pebble. Any arguments passed to pebble past 'run'\
				\nare passed to the binary. Working directory in which pebble run is\
				\nran is preserved\
				 ");
	}
	fn test()
	{
		println!("usage: pebble test [arguments..]
				\nbuilds and runs tests of the pebble. Tests are written in tests.c2\
				\npebble test is currently only available. Working directory in which pebble\
				\nis ran is preserved\
				 ");
	}
	fn scan()
	{
		println!("usage: pebble scan\
			  \n\nscans the src directory. All untracked files are are added, all non-existant\
				\nfiles are removed. The files are added to the recipe.txt, which shall not be\
				\ntouched.\
				 ");
	}
	fn add()
	{
		println!("usage pebble add [filename]\
			  \n\nadds a source file to the pebble. pebble tries either filename or src/filename,\
				\nwhichever exists.\
				 ");
	}
	fn remove()
	{
		println!("usage pebble remove [filename]\
			  \n\nremove a source file from the pebble. the file is removed from both main and test\
				\ntargets.\
				 ");
	}
	fn toml()
	{
		println!("pebble uses TOML for it's configuration file. the file has three sections:\
				\n	Package config\
				\n	Library config  (optional)\
				\n	Build config    (optional)\
			  \n\nPackage config:\
				\n	name\
				\n	version\
				\n	source_dir      (optional)\
				\n	license         (optional)\
				\n	dependencies    (optional)\
			  \n\nLibrary config:\
				\n	claim\
				\n	extra\
				\n	version         (optional)\
			  \n\nBuild config:\
				\n	pre             (optional)\
				\n	post            (optional)\
				\n	install         (optional)\
				\n	uninstall       (optional)\
			  \n\ndependencies:\
				\n	name\
				\n	version\
				 ");
	}
	match cmd.as_ref()
	{
		"new" => new(),
		"init" => init(),
		"build" => build(),
		"run" => run(),
		"test" => test(),
		"scan" => scan(),
		"add" => add(),
		"remove" => remove(),
		"toml" => toml(),
		_ => general(),
	}
	exit(0);
}
