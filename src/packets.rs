#![allow(dead_code)]
use serde_cbor;

use std::net::UdpSocket;

use errors::{fail, fail1};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Packet
{
	Publish { uname: String, hash: String, file: Vec<u8>, name: String, version: String },
	Update { data: String },
	Find { name: String, version: String },
	Upload { uname: String, hash: String, file: Vec<u8>, name: String, version: String },
	Error { msg: String },
	Register { name: String, hash: String },
	Login { name: String, hash: String },
	New,
}

impl Packet
{
	/*
	** Yay, builder pattern!
	*/
	pub fn new() -> Packet
	{
		Packet::New
	}

	/*
	** Types
	*/
	pub fn error(msg: &str) -> Packet
	{
		Packet::Error { msg: msg.to_owned() }
	}

	pub fn register(name: &str, hash: &str) -> Packet
	{
		Packet::Register
		{
			name: name.to_owned(),
			hash: hash.to_owned(),
		}
	}

	pub fn login(name: &str, hash: &str) -> Packet
	{
		Packet::Login
		{
			name: name.to_owned(),
			hash: hash.to_owned(),
		}
	}

	pub fn update(data: &str) -> Packet
	{
		Packet::Update
		{
			data: data.to_owned(),
		}
	}

	pub fn find(name: &str, version: &str) -> Packet
	{
		Packet::Find
		{
			name: name.to_owned(),
			version: version.to_owned(),
		}
	}

	pub fn upload(uname: &str, hash: &str, file: Vec<u8>, name: &str, version: &str) -> Packet
	{
		Packet::Upload
		{
			uname: uname.to_owned(),
			hash: hash.to_owned(),
			file: file,
			name: name.to_owned(),
			version: version.to_owned(),
		}
	}

	pub fn publish(uname: &str, hash: &str, file: Vec<u8>, name: &str, version: &str) -> Packet
	{
		let lib_name = "lib".to_string() + name;
		Packet::Publish
		{
			name: lib_name.to_owned(),
			hash: hash.to_owned(),
			uname: uname.to_owned(),
			file: file,
			version: version.to_owned()
		}
	}

	/*
	** Reading
	*/
	pub fn read(source: &[u8]) -> Result<Packet, serde_cbor::error::Error>
	{
		serde_cbor::de::from_slice(source)
	}

	pub fn make(self) -> Result<Vec<u8>, serde_cbor::Error>
	{
		serde_cbor::ser::to_vec(&self)
	}

	/*
	** Sending
	*/
	pub fn send(self) -> Packet
	{
		let sock = match UdpSocket::bind("0.0.0.0:0")
		{
			Ok(s) => s,
			Err(_) => fail("failed to bind to socket", 2)
		};

		if sock.connect("localhost:9001").is_err()
			{fail("failed to connect to remote host. are you connected to the internet?", 3);}

		let bytes = match self.clone().make()
		{
			Ok(b) => b,
			Err(_) => fail1("failed to serialize packet", format!("{:?}", self), 4)
		};

		loop
		{
			if sock.send(&bytes).is_err()
				{fail("failed to send data", 5);};

			let mut res_buf = [0; 64 * 1024]; // maximum response size is 60kb

			let res_size = match sock.recv(&mut res_buf)
			{
				Ok(s) => s,
				Err(_) => continue,
			};
			let res_buf = &mut res_buf[..res_size];

			let res = match Packet::read(&res_buf.to_vec())
			{
				Ok(p) => p,
				Err(_) => fail("failed to deserialize packet", 6)
			};

			return res;
		}
	}
}
