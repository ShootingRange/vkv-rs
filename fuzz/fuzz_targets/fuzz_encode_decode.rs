#![no_main]
use libfuzzer_sys::fuzz_target;
use vkv_rs::{decode, encode, Root};

fuzz_target!(|data: Root| {
    if decode(encode(&data).unwrap().as_str()).unwrap() == data {
        panic!("result did not match input");
    }
});
