pub fn br_decl32le(src: &[u8]) -> u32 {
    (src[0] as u32)
    | ((src[1] as u32) << 8)
    | ((src[2] as u32) << 16)
    | ((src[3] as u32) << 24)
}


// TODO: check efficiency
pub fn br_range_dec32le(v: &mut[u32], src: &[u8]) {
    for (i, p) in v.iter_mut().enumerate() {
        *p = br_decl32le(&src[i*4..(i+1)*4]);
    }
}
