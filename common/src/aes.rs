// TODO: remove pub qualifiers

#[inline]
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

#[inline]
pub fn br_swap32(x: u32) -> u32 {
    let x = ((x & 0x00FF00FF) << 8) // TODO: check integer promotion rules
        | ((x >> 8) & 0x00FF00FF);
    x << 16 | x >> 16
}  

#[inline]
pub fn br_enc32le(dst: &mut[u8], x: u32) {
    dst[0] = x as u8;
    dst[1] = (x >> 8) as u8;
    dst[2] = (x >> 16) as u8;
    dst[3] = (x >> 24) as u8;
}

pub fn br_range_enc32le(dst: &mut[u8], v: &[u32]) {
    for (i, p) in v.iter().enumerate() {
        br_enc32le(&mut dst[i*4..(i+1)*4], *p);
    }
}

#[allow(non_snake_case)]
pub fn br_aes_ct64_bitslice_Sbox(q: &mut[u64]) {
    /*
     * This S-box implementation is a straightforward translation of
     * the circuit described by Boyar and Peralta in "A new
     * combinational logic minimization technique with applications
     * to cryptology" (https://eprint.iacr.org/2009/191.pdf).
     *
     * Note that variables x* (input) and s* (output) are numbered
     * in "reverse" order (x0 is the high bit, x7 is the low bit).
     */
    let x0 = q[7];
    let x1 = q[6];
    let x2 = q[5];
    let x3 = q[4];
    let x4 = q[3];
    let x5 = q[2];
    let x6 = q[1];
    let x7 = q[0];

    /*
     * Top linear transformation.
     */
    let y14 = x3 ^ x5;
    let y13 = x0 ^ x6;
    let y9 = x0 ^ x3;
    let y8 = x0 ^ x5;
    let t0 = x1 ^ x2;
    let y1 = t0 ^ x7;
    let y4 = y1 ^ x3;
    let y12 = y13 ^ y14;
    let y2 = y1 ^ x0;
    let y5 = y1 ^ x6;
    let y3 = y5 ^ y8;
    let t1 = x4 ^ y12;
    let y15 = t1 ^ x5;
    let y20 = t1 ^ x1;
    let y6 = y15 ^ x7;
    let y10 = y15 ^ t0;
    let y11 = y20 ^ y9;
    let y7 = x7 ^ y11;
    let y17 = y10 ^ y11;
    let y19 = y10 ^ y8;
    let y16 = t0 ^ y11;
    let y21 = y13 ^ y16;
    let y18 = x0 ^ y16;

    /*
     * Non-linear section.
     */
    let t2 = y12 & y15;
    let t3 = y3 & y6;
    let t4 = t3 ^ t2;
    let t5 = y4 & x7;
    let t6 = t5 ^ t2;
    let t7 = y13 & y16;
    let t8 = y5 & y1;
    let t9 = t8 ^ t7;
    let t10 = y2 & y7;
    let t11 = t10 ^ t7;
    let t12 = y9 & y11;
    let t13 = y14 & y17;
    let t14 = t13 ^ t12;
    let t15 = y8 & y10;
    let t16 = t15 ^ t12;
    let t17 = t4 ^ t14;
    let t18 = t6 ^ t16;
    let t19 = t9 ^ t14;
    let t20 = t11 ^ t16;
    let t21 = t17 ^ y20;
    let t22 = t18 ^ y19;
    let t23 = t19 ^ y21;
    let t24 = t20 ^ y18;

    let t25 = t21 ^ t22;
    let t26 = t21 & t23;
    let t27 = t24 ^ t26;
    let t28 = t25 & t27;
    let t29 = t28 ^ t22;
    let t30 = t23 ^ t24;
    let t31 = t22 ^ t26;
    let t32 = t31 & t30;
    let t33 = t32 ^ t24;
    let t34 = t23 ^ t33;
    let t35 = t27 ^ t33;
    let t36 = t24 & t35;
    let t37 = t36 ^ t34;
    let t38 = t27 ^ t36;
    let t39 = t29 & t38;
    let t40 = t25 ^ t39;

    let t41 = t40 ^ t37;
    let t42 = t29 ^ t33;
    let t43 = t29 ^ t40;
    let t44 = t33 ^ t37;
    let t45 = t42 ^ t41;
    let z0 = t44 & y15;
    let z1 = t37 & y6;
    let z2 = t33 & x7;
    let z3 = t43 & y16;
    let z4 = t40 & y1;
    let z5 = t29 & y7;
    let z6 = t42 & y11;
    let z7 = t45 & y17;
    let z8 = t41 & y10;
    let z9 = t44 & y12;
    let z10 = t37 & y3;
    let z11 = t33 & y4;
    let z12 = t43 & y13;
    let z13 = t40 & y5;
    let z14 = t29 & y2;
    let z15 = t42 & y9;
    let z16 = t45 & y14;
    let z17 = t41 & y8;

    /*
     * Bottom linear transformation.
     */
    let t46 = z15 ^ z16;
    let t47 = z10 ^ z11;
    let t48 = z5 ^ z13;
    let t49 = z9 ^ z10;
    let t50 = z2 ^ z12;
    let t51 = z2 ^ z5;
    let t52 = z7 ^ z8;
    let t53 = z0 ^ z3;
    let t54 = z6 ^ z7;
    let t55 = z16 ^ z17;
    let t56 = z12 ^ t48;
    let t57 = t50 ^ t53;
    let t58 = z4 ^ t46;
    let t59 = z3 ^ t54;
    let t60 = t46 ^ t57;
    let t61 = z14 ^ t57;
    let t62 = t52 ^ t58;
    let t63 = t49 ^ t58;
    let t64 = z4 ^ t59;
    let t65 = t61 ^ t62;
    let t66 = z1 ^ t63;
    let s0 = t59 ^ t63;
    let s6 = t56 ^ !t62;
    let s7 = t48 ^ !t60;
    let t67 = t64 ^ t65;
    let s3 = t53 ^ t66;
    let s4 = t51 ^ t66;
    let s5 = t47 ^ t65;
    let s1 = t64 ^ !s3;
    let s2 = t55 ^ !t67;

    q[7] = s0;
    q[6] = s1;
    q[5] = s2;
    q[4] = s3;
    q[3] = s4;
    q[2] = s5;
    q[1] = s6;
    q[0] = s7;
}

#[inline(always)]
fn swapn(cl: u64, ch: u64, s: usize, q: &mut[u64], x: usize, y: usize){
    let a: u64 = q[x];
    let b: u64 = q[y];
    q[x] = (a & cl) | ((b & cl) << s);
    q[y] = ((a & ch) >> (s)) | (b & ch);
}

#[inline(always)]
fn swap2(q: &mut[u64], x: usize, y: usize){
    swapn(0x5555555555555555, 0xAAAAAAAAAAAAAAAA,  1, q, x, y);
}
#[inline(always)]
fn swap4(q: &mut[u64], x: usize, y: usize){
    swapn(0x3333333333333333, 0xCCCCCCCCCCCCCCCC,  2, q, x, y);
}
#[inline(always)]
fn swap8(q: &mut[u64], x: usize, y: usize){
    swapn(0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0,  4, q, x, y);
}

pub fn br_aes_ct64_ortho(q: &mut[u64]) {
    swap2(q,0,1);
    swap2(q,2,3);
    swap2(q,4,5);
    swap2(q,6,7);

    swap4(q,0,2);
    swap4(q,1,3);
    swap4(q,4,6);
    swap4(q,5,7);

    swap8(q,0,4);
    swap8(q,1,5);
    swap8(q,2,6);
    swap8(q,3,7);
}

pub fn br_aes_ct64_interleave_in(q: &mut[u64], i0: usize, i1: usize, w: &[u32]) {
    let x0 = w[0] as u64;
    let x1 = w[1] as u64;
    let x2 = w[2] as u64;
    let x3 = w[3] as u64;
    let x0 = x0 | (x0 << 16);
    let x1 = x1 | (x1 << 16);
    let x2 = x2 | (x2 << 16);
    let x3 = x3 | (x3 << 16);
    let x0 = x0 & 0x0000FFFF0000FFFF;
    let x1 = x1 & 0x0000FFFF0000FFFF;
    let x2 = x2 & 0x0000FFFF0000FFFF;
    let x3 = x3 & 0x0000FFFF0000FFFF;
    let x0 = x0 | (x0 << 8);
    let x1 = x1 | (x1 << 8);
    let x2 = x2 | (x2 << 8);
    let x3 = x3 | (x3 << 8);
    let x0 = x0 & 0x00FF00FF00FF00FF;
    let x1 = x1 & 0x00FF00FF00FF00FF;
    let x2 = x2 & 0x00FF00FF00FF00FF;
    let x3 = x3 & 0x00FF00FF00FF00FF;
    q[i0]= x0 | (x2 << 8);
    q[i1] = x1 | (x3 << 8);
}

