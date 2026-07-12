#![no_main]
use libfuzzer_sys::fuzz_target;
use dc_parser::parser::parse_program;
use std::str;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            parse_program(in_string);
        },
        Err(..)=>()
    }
});
