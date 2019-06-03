use std::slice;
extern crate libc;
use libc::size_t;

extern "C" {
    fn br_dec32le_C(src: *const u8) -> u32;
    fn br_range_dec32le_C(v: *mut u32, num: size_t, src: *const u8);
    fn br_swap32_C(x: u32) -> u32;
    fn br_enc32le_C(dst: *mut u8, x: u32);
    fn br_range_enc32le_C(dst: *mut u8, v: *const u32, num: size_t);
    fn br_aes_ct64_bitslice_Sbox_C(q: *mut u64);
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
    
    for i in 0..num {
        assert_eq!(v[i],v2[i]);
    }        
}

#[no_mangle]
pub unsafe extern "C" fn br_swap32(x: u32) -> u32 {
    let result_c = br_swap32_C(x);

    let result_rust = common::aes::br_swap32(x);
    assert_eq!(result_c, result_rust);

    result_rust
}

#[no_mangle]
pub unsafe extern "C" fn br_enc32le(dst: *mut u8, x: u32) {
    br_enc32le_C(dst, x);

    let dst: &mut [u8] = slice::from_raw_parts_mut(dst, 4);

    let mut dstv:  &mut Vec<u8> = &mut dst.to_vec();
    let dst2 = &mut dstv;
    common::aes::br_enc32le(dst2, x);
    
    for i in 0..4 {
        assert_eq!(dst[i],dst2[i]);
    }        
}

#[no_mangle]
pub unsafe extern "C" fn br_range_enc32le(dst: *mut u8, v: *const u32, num: size_t) {
    br_range_enc32le_C(dst, v, num);

    let dst: &mut [u8] = slice::from_raw_parts_mut(dst, num*4);
    let v: &[u32] = slice::from_raw_parts(v, num);

    let mut dstv:  &mut Vec<u8> = &mut dst.to_vec();
    let dst2 = &mut dstv;
    common::aes::br_range_enc32le(dst2, v);
    
    for i in 0..num {
        assert_eq!(dst[i],dst2[i]);
    }        
}

#[no_mangle]
pub unsafe extern "C" fn br_aes_ct64_bitslice_Sbox(q_raw: *mut u64) {
    let q: &mut [u64] = slice::from_raw_parts_mut(q_raw, 8);

    let mut qv:  &mut Vec<u64> = &mut q.to_vec();
    let q2 = &mut qv;
    common::aes::br_aes_ct64_bitslice_Sbox(q2);

    br_aes_ct64_bitslice_Sbox_C(q_raw);

    for i in 0..8 {
        assert_eq!(q[i],q2[i]);
    }        
}
