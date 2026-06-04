#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::{ByteSlice};

fuzz_target!(|data: &[u8]| {
    let _ = data.to_str();
    let mut iter = data.char_indices();
    while let Some((_i, _e, _ch)) = iter.next_back() {}
});
