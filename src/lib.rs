//     qr_builder
//     Copyright (C) 2020  Giancarlo DiMino
//     This program is free software; you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation; either version 2 of the License, or
//     (at your option) any later version.
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//     You should have received a copy of the GNU General Public License along
//     with this program; if not, write to the Free Software Foundation, Inc.,
//     51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
use image::Rgba;
use lodepng::Encoder;
use qrcode::QrCode;
use qrcode::types::QrError;
use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;
use pyo3::wrap_pyfunction;
use std::convert::TryInto;

/// Module for generating QR Codes.
#[pymodule]
fn qr_builder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(gen_qr_code))?;

    Ok(())
}

#[pyfunction]
#[text_signature = "(text)"]
/// Generates PNG QR Code based off of given string
fn gen_qr_code(text: String) -> PyResult<Vec<u8>> {
    let code = match QrCode::new(text.as_bytes()) {
        Ok(c) => c,
        Err(e) => return Err(PyErr::new::<PyIOError, _>(qrerror_string(e)))
    };
    let buf = code.render::<Rgba<u8>>().build();

    let mut enc = Encoder::new();
    let width = buf.width().try_into()?;
    let height = buf.height().try_into()?;
    match enc.encode(&buf, width, height) {
        Ok(byte_vec) => Ok(byte_vec),
        Err(e) => Err(PyErr::new::<PyIOError, _>(e.to_string()))
    }
}

fn qrerror_string(e: QrError) -> &'static str {
    match e {
        QrError::DataTooLong => "The data is too long to encode into a QR code for the given version.",
        QrError::InvalidVersion => "The provided version / error correction level combination is invalid.",
        QrError::UnsupportedCharacterSet => "Some characters in the data cannot be supported by the provided QR code version.",
        QrError::InvalidEciDesignator => "The provided ECI designator is invalid. A valid designator should be between 0 and 999999.",
        QrError::InvalidCharacter => "A character not belonging to the character set is found."
    }
}

// Refer to https://pyo3.rs/v0.9.2/advanced.html
// Run tests with: cargo +nightly test --release --no-default-features
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_png_header() {
        let data = match gen_qr_code("Who ya gonna call?".to_string()) {
            Ok(d) => d,
            Err(e) => {
                println!("There was a problem creating the image data: {}", e.to_string());
                return
            }
        };
        assert_eq!(&data[..8], &vec![137u8, 80, 78, 71, 13, 10, 26, 10][..]);
    }
}
