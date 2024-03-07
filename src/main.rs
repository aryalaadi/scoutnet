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
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::time::Duration;

use clap::Parser;

static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    ipaddr: String,
    #[arg(short, long, default_value = "0")]
    start_port: i32,
    #[arg(short, long, default_value = "8000")]
    end_port: i32,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    multithreaded: bool,
}

fn scan_port(addr: String, port: i32, verbose: bool) {
    GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    thread::spawn(move || {
        let current_addr = format!("{}:{}", addr, port.to_string().to_owned());
        if let Ok(_stream) = TcpStream::connect_timeout(
            &current_addr.to_socket_addrs().unwrap().next().unwrap(),
            Duration::new(1, 0),
        ) {
            println!("{}\topen\t{}", port, "TODO");
        } else {
            if verbose {
                println!("{}\tclose\t{}", port, "TODO");
            }
        }
        GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
    });
}

fn scan_port_wait(addr: String, port: i32, verbose: bool) {
    let current_addr = format!("{}:{}", addr, port.to_string().to_owned());
    if let Ok(_stream) = TcpStream::connect_timeout(
		&current_addr.to_socket_addrs().unwrap().next().unwrap(),
        Duration::new(1, 0),
    ) {
        println!("{}\topen\t{}", port, "TODO");
    } else {
        if verbose {
            println!("{}\tclose\t{}", port, "TODO");
        }
    }
}
fn main() {
    let args = Args::parse();

    let addr = args.ipaddr.clone();
    let port_scan_start = args.start_port.clone();
    let port_scan_end = args.end_port.clone();
    let verbose = args.verbose;
    let multithreaded_run = args.multithreaded;

    println!("PORT\tSTATE\tSERVICE");
    if !multithreaded_run {
        for i in port_scan_start..port_scan_end {
            scan_port_wait(addr.clone(), i, verbose);
        }
    } else {
        for i in port_scan_start..port_scan_end {
            scan_port(addr.clone(), i, verbose);
        }
    }

    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }
}
