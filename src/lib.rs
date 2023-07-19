mod utils;

use std::borrow::Cow;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

const TABLE: [u32; 5] = [85 * 85 * 85 * 85, 85 * 85 * 85, 85 * 85, 85, 1];
const ERR_MSG_CHUNK_LENGTH: &str = "Chunk length error: expected 4 bytes";
const ERR_MSG_MISALIGNED: &str = "Missaligned z in input";
const ERR_MSG_OUT_OF_RANGE: &str = "Input char is out of range for Ascii85";
const ERR_MSG_INVALID_UTF8: &str = "Invalid UTF-8 sequence";

#[wasm_bindgen]
pub fn encode(input: &str) -> Result<String, JsError> {
    let input_arr = input.as_bytes();
    let mut result = String::with_capacity(5 * (input_arr.len() / 4 + 16));

    result.push_str("<~");

    for chunk in input_arr.chunks(4) {
        let (chunk, count) = if chunk.len() == 4 {
            (Cow::from(chunk), 5)
        } else {
            let mut new_chunk = Vec::new();
            new_chunk.resize_with(4, || 0);
            new_chunk[..chunk.len()].copy_from_slice(chunk);
            (Cow::from(new_chunk), 5 - (4 - chunk.len()))
        };

        let number = match chunk.as_ref().try_into() {
            Ok(v) => u32::from_be_bytes(v),
            Err(_) => return Err(JsError::new(ERR_MSG_CHUNK_LENGTH)),
        };

        for i in 0..count {
            let digit = (((number / TABLE[i]) % 85) + 33) as u8;
            result.push(digit as char);
        }
    }

    result.push_str("~>");

    Ok(result)
}

fn decode_digit(digit: u8, counter: &mut usize, chunk: &mut u32, result: &mut Vec<u8>) {
    let byte = digit - 33;

    *chunk += byte as u32 * TABLE[*counter];

    if *counter == 4 {
        result.extend_from_slice(&chunk.to_be_bytes());
        *chunk = 0;
        *counter = 0;
    } else {
        *counter += 1;
    }
}

#[wasm_bindgen]
pub fn decode(input: &str) -> Result<String, JsError> {
    let mut result = Vec::with_capacity(4 * (input.len() / 5 + 16));

    let mut counter = 0;
    let mut chunk = 0;

    for digit in input
        .trim_start()
        .trim_start_matches("<~")
        .trim_end()
        .trim_end_matches("~>")
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
    {
        if digit == b'z' {
            if counter == 0 {
                result.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                return Err(JsError::new(ERR_MSG_MISALIGNED).into());
            }
        }

        if digit < 33 || digit > 117 {
            return Err(JsError::new(ERR_MSG_OUT_OF_RANGE).into());
        }

        decode_digit(digit, &mut counter, &mut chunk, &mut result);
    }

    let mut to_remove = 0;

    while counter != 0 {
        decode_digit(b'u', &mut counter, &mut chunk, &mut result);
        to_remove += 1;
    }

    result.drain((result.len() - to_remove)..result.len());

    Ok(String::from_utf8(result).map_err(|_| JsError::new(ERR_MSG_INVALID_UTF8))?)
}
