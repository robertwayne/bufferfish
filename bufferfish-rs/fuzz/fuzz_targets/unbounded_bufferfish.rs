#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut bf = bufferfish::Bufferfish::with_capacity(0);

    if let Err(_) = bf.write_raw_bytes(data) {
        return;
    }

    if let Err(_) = bf.read_u8() {
        return;
    }
});
