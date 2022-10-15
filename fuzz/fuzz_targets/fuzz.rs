#![no_main]
use libfuzzer_sys::fuzz_target;
use bstr::BStr;

fuzz_target!(|data: &[u8]| {
    let a = BStr::new(data);
    a.replace("a", "b");
});