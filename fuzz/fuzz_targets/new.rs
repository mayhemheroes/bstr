#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::BStr;

fuzz_target!(|data: &[u8]| {
    let _ = BStr::new(data);
});