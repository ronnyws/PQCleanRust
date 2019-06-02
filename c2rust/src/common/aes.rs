use std::slice;

extern "C" {
    fn br_dec32le_C(src: *const u8) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn br_dec32le(src: *const u8) -> u32 {
    let result_c = br_dec32le_C(src);

    let src: &[u8] = slice::from_raw_parts(src, 4 as usize);

    let result_rust = common::aes::br_decl32le(src);
    assert!(result_c == result_rust);

    result_rust
}
