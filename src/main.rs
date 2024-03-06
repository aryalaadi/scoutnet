/*
    scoutnet: port scanning utility for UNIX-like systems
    Copyright (C) 2023 aryalaadi123 @ gmail.com

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/


use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::Duration;

fn scan_port(addr: String, port: i32) {
		thread::spawn(move || {
			let current_addr = format!("{}:{}", addr, port.to_string().to_owned());
			if let Ok(stream) = TcpStream::connect_timeout(&current_addr.to_socket_addrs().unwrap().next().unwrap(), Duration::new(1, 0)) {
				println!("{}\topen\t{}", port, "TODO");
			}
		});
}

fn main() {
	let mut args = std::env::args();

	if args.len() != 4 {
		println!("usage: \n\tscoutnet ip port_start port_end");
		std::process::exit(127);
	}
	
	args.next();
	let addr = args.next().unwrap();
	let port_scan_start = args.next().unwrap().parse::<i32>().unwrap();
	let port_scan_end = args.next().unwrap().parse::<i32>().unwrap();

	println!("PORT\tSTATE\tSERVICE");
	for i in port_scan_start..port_scan_end {
		scan_port(addr.clone(), i);
	}
}
