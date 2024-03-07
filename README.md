# Scoutnet
A simple port scanning utility for UNIX-like systems 

## building:
to build scoutnet make sure you have rust installed
```
cargo build --release
```	

## usage: 
```
Usage: scoutnet [OPTIONS]

Options:
  -i, --ipaddr <IPADDR>          [default: 0.0.0.0]
  -s, --start-port <START_PORT>  [default: 0]
  -e, --end-port <END_PORT>      [default: 8000]
  -v, --verbose
  -m, --multithreaded
  -h, --help                     Print help
  -V, --version                  Print version
```

note: multithreaded scan might get you banned from networks, use this tool responsibly
