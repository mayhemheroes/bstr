#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::{BStr, BString, ByteSlice};

fuzz_target!(|data: &[u8]| {
    let bstr = BStr::new(data);
    let _ = bstr.len();
    let _ = bstr.is_empty();
    let bstring = BString::from(data);
    let _ = bstring.len();
    let _ = bstring.to_str();
});
