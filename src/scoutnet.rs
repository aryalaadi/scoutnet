/*
    libscoutnet: port scanning library for rust
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

use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

fn check_service_on_stream(mut stream: TcpStream) -> String {
    // check for HTTP
    let mut buf = [0; 4];
    stream.write("GET / HTTP/1.1\n\n".as_bytes()).unwrap();
    let _ = stream.read_exact(&mut buf);
    if String::from_utf8_lossy(&buf) == "HTTP" {
        return "HTTP".to_string();
    // check for SSH
    } else if String::from_utf8_lossy(&buf) == "SSH-" {
        return "SSH".to_string();
    } else {
        return "UNKNOWN".to_string();
    }
}

pub fn scan_port(addr: String, port: i32, _verbose: bool) -> (String, bool) {
    let current_addr = format!("{}:{}", addr, port.to_string().to_owned());
    if let Ok(stream) = TcpStream::connect_timeout(
        &current_addr.to_socket_addrs().unwrap().next().unwrap(),
        Duration::new(1, 0),
    ) {
        return (check_service_on_stream(stream), true);
    } else {
        return ("".to_string(), false);
    }
}
