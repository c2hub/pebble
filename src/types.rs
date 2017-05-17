pub enum PebbleType
{
	Executable,
	SharedLib,
	StaticLib,
}

impl ToString for PebbleType
{
	fn to_string(&self) -> String
	{
		match *self
		{
			PebbleType::Executable => "executable".to_string(),
			PebbleType::SharedLib =>  "shared library".to_string(),
			PebbleType::StaticLib => "static library".to_string()
		}
	}
}
