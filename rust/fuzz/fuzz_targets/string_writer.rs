#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut bf = bufferfish::Bufferfish::with_capacity(0);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = bf.write_string(s);
    }
});
