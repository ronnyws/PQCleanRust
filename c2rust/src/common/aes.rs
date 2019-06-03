use std::slice;
extern crate libc;
use libc::size_t;

extern "C" {
    fn br_dec32le_C(src: *const u8) -> u32;
    fn br_range_dec32le_C(v: *mut u32, num: size_t, src: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn br_dec32le(src: *const u8) -> u32 {
    let result_c = br_dec32le_C(src);

    let src: &[u8] = slice::from_raw_parts(src, 4 as usize);

    let result_rust = common::aes::br_decl32le(src);
    assert!(result_c == result_rust);

    result_rust
}

#[no_mangle]
pub unsafe extern "C" fn br_range_dec32le(v: *mut u32, num: size_t, src: *const u8) {
    br_range_dec32le_C(v, num, src);

    let v: &mut [u32] = slice::from_raw_parts_mut(v, num);
    let src: &[u8] = slice::from_raw_parts(src, num*4 as usize);

    let mut vv:  &mut Vec<u32> = &mut v.to_vec();
    let v2 = &mut vv;
    common::aes::br_range_dec32le(v2, src);
    
    for (i, p) in v.iter_mut().enumerate() {
        assert_eq!(*p,v2[i]);
    }        
}
