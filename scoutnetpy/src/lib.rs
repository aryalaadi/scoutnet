/*
    scoutnetpy: port scanning library for python3
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

use libscoutnet::scoutnet::scan_port;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn scanport(addr: String, port: i32) -> PyResult<(String, bool)> {
    Ok(scan_port(addr, port, false))
}

/// A Python module implemented in Rust.
#[pymodule]
fn scoutnetpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scanport, m)?)?;
    Ok(())
}
