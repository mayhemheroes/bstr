#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::decode_utf8;

fuzz_target!(|data: &[u8]| {
    let (_, _) = decode_utf8(data);
});