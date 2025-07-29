#![no_main]
use libfuzzer_sys::fuzz_target;
use flameview::add_one;

fuzz_target!(|data: &[u8]| {
    if data.len() == 4 {
        let v = i32::from_le_bytes(data.try_into().unwrap());
        let _ = add_one(v);
    }
});
