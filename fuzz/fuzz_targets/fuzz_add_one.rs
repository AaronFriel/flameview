#![no_main]
use flameview::add_one;
use libafl_libfuzzer::fuzz;

fuzz!(|data: &[u8]| {
    if data.len() == 4 {
        let v = i32::from_le_bytes(data.try_into().unwrap());
        let _ = add_one(v);
    }
});
