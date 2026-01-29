//@ build-fail

#![feature(repr_simd, intrinsics)]

#[repr(simd)]
struct Simd<const N: usize>([f32; N]);

fn main() {
    let _empty = Simd::<0>([]); //~ ERROR the SIMD type `Simd<const { 0 }>` has zero elements
}
