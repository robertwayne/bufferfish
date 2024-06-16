#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut bf = bufferfish::Bufferfish::new();

    if bf.write_raw_bytes(data).is_err() {
        return;
    }

    if bf.read_u8().is_err() {
        return;
    }
});
