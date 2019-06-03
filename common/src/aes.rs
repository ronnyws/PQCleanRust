// TODO: remove pub qualifiers
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

pub fn br_swap32(x: u32) -> u32 {
    let x = ((x & 0x00FF00FF) << 8) // TODO: check integer promotion rules
        | ((x >> 8) & 0x00FF00FF);
    x << 16 | x >> 16
}  
