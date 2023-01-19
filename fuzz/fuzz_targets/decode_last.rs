#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::decode_last_utf8;

fuzz_target!(|data: &[u8]| {
    let _ = decode_last_utf8(data);
});