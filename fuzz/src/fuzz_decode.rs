#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::{ByteSlice};

fuzz_target!(|data: &[u8]| {
    let _ = data.to_str();
    for chunk in data.utf8_chunks() {
        let _ = chunk.valid();
        let _ = chunk.invalid();
    }
});
