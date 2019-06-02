pub fn br_decl32le(src: &[u8]) -> u32 {
    (src[0] as u32)
    | ((src[1] as u32) << 8)
    | ((src[2] as u32) << 16)
    | ((src[3] as u32) << 24)
}
