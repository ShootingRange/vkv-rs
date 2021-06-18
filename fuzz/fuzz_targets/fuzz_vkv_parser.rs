#![no_main]
use libfuzzer_sys::fuzz_target;
use vkv_rs::encode;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        parse_vkv(s);
    }
});
