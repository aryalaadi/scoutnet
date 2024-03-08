# Scoutnet: A simple port scanning utility for UNIX-like systems 

## this repo provides the following:
- scoutnet binary
- libscoutnet Rust library  
- scoutnetpy wrapping libscoutnet for Python3

## building and installing the binary:
to build scoutnet make sure you have rust installed
```
make  
sudo make install
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

## python3 
build the python3 library PyO3's maturin
```
(.env) ~/docs/projects/irl/scoutnet λ cd scoutnetpy
(.env) ~/docs/projects/irl/scoutnet λ maturin develop
(.env) ~/docs/projects/irl/scoutnet λ python3
Python 3.12.2 (main, Feb 15 2024, 06:47:30) [GCC 13.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import scoutnetpy
>>> scoutnetpy.scanport("0.0.0.0", 22)
('SSH', True)
```

note: multithreaded scan might get you banned from networks, use this tool responsibly
