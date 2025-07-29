#![no_main]
use libafl_libfuzzer::fuzz;
use flameview::add_one;

fuzz!(|data: &[u8]| {
    if data.len() == 4 {
        let v = i32::from_le_bytes(data.try_into().unwrap());
        let _ = add_one(v);
    }
});
