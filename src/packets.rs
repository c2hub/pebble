#![allow(dead_code)]
use serde_cbor;

use std::net::UdpSocket;
use std::process::exit;

use errors::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Packet
{
	pub ptype: PacketType,
	pub name: Option<String>,
	pub extra: Option<String>,
	pub data: Option<String>,
	pub data2: Option<String>,
	pub raw_data: Option<Vec<u8>>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PacketType
{
	Publish,
	Update,
	Find,
	Upload,
	Error,
	Register,
	Login,
	New,
}

impl Packet
{
	/*
	** Yay, builder pattern!
	*/
	pub fn new() -> Packet
	{
		Packet
		{
			ptype: PacketType::New,
			name: None,
			extra: None,
			data: None,
			data2: None,
			raw_data: None
		}
	}

	pub fn name(mut self, name: String) -> Packet
	{
		self.name = Some(name);
		self
	}

	pub fn ptype(mut self, ptype: PacketType) -> Packet
	{
		self.ptype = ptype;
		self
	}

	pub fn extra(mut self, extra: String) -> Packet
	{
		self.extra = Some(extra);
		self
	}

	pub fn data(mut self, data: String) -> Packet
	{
		self.data = Some(data);
		self
	}

	pub fn data2(mut self, data: String) -> Packet
	{
		self.data2 = Some(data);
		self
	}

	pub fn raw_data(mut self, raw_data: Vec<u8>) -> Packet
	{
		self.raw_data = Some(raw_data);
		self
	}

	/*
	** Types
	*/
	pub fn error(msg: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Error)
			.name(msg.to_owned())
	}

	pub fn register(name: &str, hash: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Register)
			.name(name.to_owned())
			.data(hash.to_owned())
	}

	pub fn login(name: &str, hash: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Login)
			.name(name.to_owned())
			.data(hash.to_owned())
	}

	pub fn update(name: &str, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Update)
			.name(name.to_owned())
			.data(version.to_owned())
	}

	pub fn find(name: &str, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Find)
			.name(name.to_owned())
			.data(version.to_owned())
	}

	pub fn upload(uname: &str, hash: &str, file: Vec<u8>, name: &str, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Upload)
			.name(name.to_owned())
			.raw_data(file)
			.data(version.to_owned())
			.data2(uname.to_owned())
			.extra(hash.to_owned())
	}

	pub fn publish(name: &str, file: Vec<u8>, version: &str) -> Packet
	{
		let lib_name = "lib".to_string() + name;
		Packet::new()
			.ptype(PacketType::Publish)
			.name(lib_name)
			.raw_data(file)
			.data(version.to_owned())
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
		let sock = match UdpSocket::bind("0.0.0.0:9001")
		{
			Ok(s) => s,
			Err(_) => fail("failed to bind to socket", 2)
		};

		if sock.connect("magnusi.tech:9001").is_err()
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

			let mut res_buf = [0; 2 * 1024 * 1024]; // maximum response size is 2mb

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
